use crate::token_output_stream::TokenOutputStream;
use anyhow::Error as E;
use candle_core::{DType, Device, Result as Res, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::qwen2::{Config as ConfigBase, ModelForCausalLM as ModelBase};
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;

const QWEN_MODEL: &str = "Qwen/Qwen2.5-0.5B-Instruct";

pub struct TextGeneration {
    model: ModelBase,
    device: Device,
    tokenizer: TokenOutputStream,
    logits_processor: LogitsProcessor,
    repeat_penalty: f32,
    repeat_last_n: usize,
}

impl TextGeneration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        model: ModelBase,
        tokenizer: Tokenizer,
        seed: u64,
        temp: Option<f64>,
        top_p: Option<f64>,
        repeat_penalty: f32,
        repeat_last_n: usize,
        device: &Device,
    ) -> Self {
        let logits_processor = LogitsProcessor::new(seed, temp, top_p);
        Self {
            model,
            tokenizer: TokenOutputStream::new(tokenizer),
            logits_processor,
            repeat_penalty,
            repeat_last_n,
            device: device.clone(),
        }
    }

    pub fn run(&mut self, prompt: &str, sample_len: usize) -> Result<String, E> {
        use std::io::Write;
        self.tokenizer.clear();
        let mut tokens = self
            .tokenizer
            .tokenizer()
            .encode(prompt, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();
        for &t in tokens.iter() {
            if let Some(t) = self.tokenizer.next_token(t)? {
                print!("{t}")
            }
        }
        std::io::stdout().flush()?;

        let mut llm_output: String = String::new();

        let mut generated_tokens = 0usize;
        let eos_token = match self.tokenizer.get_token("<|endoftext|>") {
            Some(token) => token,
            None => anyhow::bail!("cannot find the <|endoftext|> token"),
        };
        let start_gen = std::time::Instant::now();
        for index in 0..sample_len {
            let context_size = if index > 0 { 1 } else { tokens.len() };
            let start_pos = tokens.len().saturating_sub(context_size);
            let ctxt = &tokens[start_pos..];
            let input = Tensor::new(ctxt, &self.device)?.unsqueeze(0)?;
            let logits = self.model.forward(&input, start_pos)?;
            let logits = logits.squeeze(0)?.squeeze(0)?.to_dtype(DType::F32)?;
            let logits = if self.repeat_penalty == 1. {
                logits
            } else {
                let start_at = tokens.len().saturating_sub(self.repeat_last_n);
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    self.repeat_penalty,
                    &tokens[start_at..],
                )?
            };

            let next_token = self.logits_processor.sample(&logits)?;
            tokens.push(next_token);
            generated_tokens += 1;
            if next_token == eos_token {
                break;
            }
            if let Some(t) = self.tokenizer.next_token(next_token)? {
                llm_output.push_str(&t);
                print!("{t}");
                std::io::stdout().flush()?;
            }
        }
        let dt = start_gen.elapsed();
        if let Some(rest) = self.tokenizer.decode_rest().map_err(E::msg)? {
            llm_output.push_str(&rest);
            print!("{rest}");
        }
        print!("\n");

        std::io::stdout().flush()?;
        println!(
            "\n{generated_tokens} tokens generated ({:.2} token/s)",
            generated_tokens as f64 / dt.as_secs_f64(),
        );
        Ok(llm_output)
    }

    pub fn run_once(&mut self, prompt: &str, index: usize) -> Result<Option<String>, E> {
        if index == 0 {
            self.tokenizer.clear();
            self.model.clear_kv_cache();
        }
        let mut tokens = self
            .tokenizer
            .tokenizer()
            .encode(prompt, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();

        let eos_token = match self.tokenizer.get_token("<|endoftext|>") {
            Some(token) => token,
            None => anyhow::bail!("cannot find the <|endoftext|> token"),
        };
        let context_size = if index > 0 { 1 } else { tokens.len() };
        let start_pos = tokens.len().saturating_sub(context_size);
        let ctxt = &tokens[start_pos..];
        let input = Tensor::new(ctxt, &self.device)?.unsqueeze(0)?;

        let logits = self.model.forward(&input, start_pos)?;
        let logits = logits.squeeze(0)?.squeeze(0)?.to_dtype(DType::F32)?;
        let logits = if self.repeat_penalty == 1. {
            logits
        } else {
            let start_at = tokens.len().saturating_sub(self.repeat_last_n);
            candle_transformers::utils::apply_repeat_penalty(
                &logits,
                self.repeat_penalty,
                &tokens[start_at..],
            )?
        };

        let next_token = self.logits_processor.sample(&logits)?;
        tokens.push(next_token);
        if next_token == eos_token {
            return Ok(None);
        }
        if let Some(t) = self.tokenizer.next_token(next_token)? {
            return Ok(Some(t));
        }
        match self.tokenizer.decode_rest().map_err(E::msg)? {
            Some(rest) => Ok(Some(rest)),
            None => Ok(None),
        }
    }
}

pub fn create_pipeline() -> Result<TextGeneration, E> {
    let api = Api::new()?;
    let repo = api.repo(Repo::with_revision(
        QWEN_MODEL.to_string(),
        RepoType::Model,
        "main".to_string(),
    ));

    let tokenizer_file = repo.get("tokenizer.json")?;
    let filenames = vec![repo.get("model.safetensors")?];
    // let filenames = hub_load_safetensors(&repo, "model.safetensors.index.json")?;
    let tokenizer = Tokenizer::from_file(tokenizer_file).map_err(E::msg)?;
    let config_file = repo.get("config.json")?;
    let device = Device::Cpu;
    let dtype = if device.is_cuda() {
        DType::BF16
    } else {
        DType::F32
    };
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&filenames, dtype, &device)? };
    let config: ConfigBase = serde_json::from_slice(&std::fs::read(config_file)?)?;
    let model = ModelBase::new(&config, vb)?;

    let pipeline = TextGeneration::new(
        model,
        tokenizer,
        //Logits seed
        293772458,
        //Temperature
        Some(0.),
        //Top_p
        Some(3.),
        //Repeat penalty
        1.4,
        //Repeat last n
        64,
        &device,
    );

    Ok(pipeline)
}

// fn hub_load_safetensors(
//     repo: &hf_hub::api::sync::ApiRepo,
//     json_file: &str,
// ) -> Res<Vec<std::path::PathBuf>> {
//     let json_file = repo.get(json_file).map_err(candle_core::Error::wrap)?;
//     let json_file = std::fs::File::open(json_file)?;
//     let json: serde_json::Value =
//         serde_json::from_reader(&json_file).map_err(candle_core::Error::wrap)?;
//     let weight_map = match json.get("weight_map") {
//         None => candle_core::bail!("no weight map in {json_file:?}"),
//         Some(serde_json::Value::Object(map)) => map,
//         Some(_) => candle_core::bail!("weight map in {json_file:?} is not a map"),
//     };
//     let mut safetensors_files = std::collections::HashSet::new();
//     for value in weight_map.values() {
//         if let Some(file) = value.as_str() {
//             safetensors_files.insert(file.to_string());
//         }
//     }
//     let safetensors_files = safetensors_files
//         .iter()
//         .map(|v| repo.get(v).map_err(candle_core::Error::wrap))
//         .collect::<Res<Vec<_>>>()?;
//     Ok(safetensors_files)
// }
