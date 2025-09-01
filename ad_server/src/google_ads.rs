use std::sync::Arc;

use crate::parse_conversation_into_topics;

static GOOGLE_AD_ANONYMOUS: &str = "Find more about {} on [https://google.com](Google)";
static GOOGLE_AD_TARGETED: &str =
    "Hi {}! You can find more about this topic on [https://google.com](Google)";

// fn parse_conversation_into_topics(conv: String) -> String {
//     let stop_words = get(LANGUAGE::English);
//     let yake = Yake::new(YakeParams::WithDefaults(conv.as_str(), &stop_words));
//     let ranked_keywords: Vec<String> = yake.get_ranked_keywords(10);
//     ranked_keywords[0].clone()
// }

pub fn get_ad(data: crate::ThirdPartyProcessorData, rake: Arc<rake::Rake>) -> String {
    match data.username {
        None => format!(
            "Find more about {} on [Google](https://google.com)",
            parse_conversation_into_topics(data.prompt, rake)
        ),
        Some(username) => format!(
            "Hi {}! You can find more about {} on [Google](https://google.com)",
            username,
            parse_conversation_into_topics(data.prompt, rake)
        ),
    }
}
