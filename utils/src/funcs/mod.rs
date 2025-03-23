use crate::types::inference_types::{LLMError, Message};

pub fn validate_user(role: String) -> Result<String, LLMError> {
    match role.as_str() {
        role @ ("user" | "model") => Ok(role.to_string()),
        // role @ ("user" | "assistant") => Ok(role.to_string()),
        role @ _ => {println!("Role is not that expected: Got {}", role); Err(LLMError::ValidationError)},
    }
}

pub fn validate_body(body: String) -> Result<String, LLMError> {
    // match !(body.contains("<|im_start|>") || body.contains("<|im_end|>")) {
    match !(body.contains("<start_of_turn>") || body.contains("<end_of_turn>")) {
        true => Ok(body),
        false =>{ println!("Body is : {}", body); Err(LLMError::ValidationError)},
    }
}

pub fn parse_message(message: Message) -> Result<String, LLMError> {
    Ok(format!(
        "<start_of_turn>{}\n{}<end_of_turn>\n",
        validate_user(message.role)?,
        validate_body(message.content)?
    ))
}

// pub fn parse_message(message: Message) -> Result<String, LLMError> {
//     Ok(format!(
//         "<|im_start|>{}\n{}<|im_end|>\n",
//         validate_user(message.role)?,
//         validate_body(message.content)?
//     ))
// }

pub fn parse_conversation(conv: Vec<Message>) -> Result<String, LLMError> {
    conv.into_iter()
        .map(|x| parse_message(x.clone()))
        .collect::<Result<Vec<_>, LLMError>>()
        .map(|mut vec| {
            vec.push("<start_of_turn>model\n".to_string());
            vec
        })
        .map(|vec| vec.join(""))
}

pub fn parse_stored_conversation(stored_conv: String) -> Result<Vec<Message>, LLMError> {
    let mut messages = Vec::new();
    let parts = stored_conv.split("<start_of_turn>").collect::<Vec<_>>();

    for part in parts.iter().skip(1) {
        match part.split_once('\n') {
            Some((role, rest)) => match rest.split_once("<end_of_turn>") {
                Some((content, _)) => messages.push(Message {
                    role: role.to_string(),
                    content: content.trim_end_matches('\n').to_string(),
                }),
                None => return Err(LLMError::ValidationError),
            },
            None => return Err(LLMError::ValidationError),
        }
    }
    Ok(messages)
}
