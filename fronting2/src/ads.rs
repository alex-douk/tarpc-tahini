use alohomora::{
    bbox::BBox,
    context::Context,
    rocket::{
        BBoxCookie, BBoxCookieJar, BBoxJson, BBoxRedirect, JsonResponse, RequestBBoxJson,
        ResponseBBoxJson, route,
    },
};
use services_utils::policies::marketing_policy::THIRD_PARTY_PROCESSORS;
use services_utils::policies::shared_policies::UsernamePolicy;
use std::collections::HashMap;

#[route(GET, "/get_vendors")]
pub(crate) async fn get_ads_vendors() -> JsonResponse<Vec<String>, ()> {
    JsonResponse(
        Vec::from(THIRD_PARTY_PROCESSORS)
            .iter()
            .map(|x| x.to_string())
            .collect(),
        Context::empty(),
    )
}
