use alohomora::bbox::BBox as PCon;
use alohomora::fold::fold;
use alohomora::pure::PrivacyPureRegion as PPR;
use services_utils::policies::MarketingPolicy;
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

pub fn get_ad(data: crate::ThirdPartyProcessorData) -> PCon<String, MarketingPolicy> {
    match data.username {
        None => data.prompt.into_ppr(PPR::new(|conv| {
            format!(
                "Find more about {} on [Google](https://google.com)",
                parse_conversation_into_topics(conv)
            )
        })),
        Some(username) => fold((username, data.prompt))
            .unwrap()
            .into_ppr(PPR::new(|(uname_unboxed, conv_unboxed)| {
                format!(
                    "Hi {}! You can find more about {} on [Google](https://google.com)",
                    uname_unboxed,
                    parse_conversation_into_topics(conv_unboxed)
                )
            }))
            .specialize_policy()
            .expect("Couldn't coerce ad policies together in Google"),
    }
}
