use alohomora::bbox::BBox as PCon;
use alohomora::fold::fold;
use alohomora::pure::PrivacyPureRegion as PPR;
use services_utils::policies::MarketingPolicy;

fn parse_conversation_into_topics(conv: String) -> String {
    //TODO(douk): Add some cool stuff about NLP methods
    return "this subject".to_string();
}

pub fn get_ad(data: crate::ThirdPartyProcessorData) -> PCon<String, MarketingPolicy> {
    match data.username {
        None => data.prompt.into_ppr(PPR::new(|conv| {
            format!(
                "More people discussing {} on [Facebook](https://facebook.com)",
                parse_conversation_into_topics(conv)
            )
        })),
        Some(username) => fold((username, data.prompt))
            .unwrap()
            .into_ppr(PPR::new(|(uname_unboxed, conv_unboxed)| {
                format!(
                    "Hi {}! You can find more people discussing {} on [Facebook](https://facebook.com)",
                    uname_unboxed,
                    parse_conversation_into_topics(conv_unboxed)
                )
            }))
            .specialize_policy()
            .expect("Couldn't coerce ad policies together in Meta"),
    }
}
