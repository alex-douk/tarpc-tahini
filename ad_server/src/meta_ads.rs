use std::sync::Arc;

use advertisement_tahini_utils::policies::MarketingPolicy;
use rake::Rake;
use sesame::fold::fold;
use sesame::pcon::PCon;
use sesame::policy::AnyPolicyDyn;
use sesame::verified::VerifiedRegion as VR;
// use keyword_extraction::yake::{Yake, YakeParams};
// use stop_words::{get, LANGUAGE};
use crate::parse_conversation_into_topics;

// fn parse_conversation_into_topics(conv: String) -> String {
//     let stop_words = get(LANGUAGE::English);
//     let yake = Yake::new(YakeParams::WithDefaults(conv.as_str(), &stop_words));
//     let ranked_keywords: Vec<String> = yake.get_ranked_keywords(10);
//     ranked_keywords[0].clone()
// }

pub fn get_ad(
    data: crate::ThirdPartyProcessorData,
    rake: Arc<Rake>,
) -> PCon<String, MarketingPolicy> {
    match data.username {
        None => data.prompt.into_verified(VR::new(|conv| {
            format!(
                "More people discussing {} on [Facebook](https://facebook.com)",
                parse_conversation_into_topics(conv, rake)
            )
        })),
        Some(username) => fold::<dyn AnyPolicyDyn, _>((username, data.prompt))
            .unwrap()
            .into_verified(VR::new(|(uname_unboxed, conv_unboxed)| {
                format!(
                    "Hi {}! You can find more people discussing {} on [Facebook](https://facebook.com)",
                    uname_unboxed,
                    parse_conversation_into_topics(conv_unboxed, rake)
                )
            }))
            .specialize_policy()
            .expect("Couldn't coerce ad policies together in Meta"),
    }
}
