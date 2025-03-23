use crate::database::fetch_or_insert_user;
use alohomora::{
    bbox::BBox,
    context::Context,
    rocket::{route, BBoxCookieJar, BBoxJson, JsonResponse, RequestBBoxJson, ResponseBBoxJson},
};
use services_utils::policies::shared_policies::UsernamePolicy;
use std::collections::HashMap;

#[derive(Clone, RequestBBoxJson)]
pub struct LoginForm {
    username: BBox<String, UsernamePolicy>,
}

#[derive(Clone, ResponseBBoxJson)]
pub struct LoginResponse {
    uuid: BBox<String, UsernamePolicy>,
}

//TODO(douk): Transform this into a proper cookie generation endpoint
#[route(POST, "/", data = "<data>")]
pub(crate) async fn login(
    // cookies: BBoxCookieJar<'_,'_>,
    data: BBoxJson<LoginForm>,
) -> alohomora::rocket::JsonResponse<LoginResponse, ()> {
    // let is_authenticated = cookies.get(name)
    let resp = LoginResponse {
        uuid: fetch_or_insert_user(data.username.clone()).await,
    };
    JsonResponse(resp, Context::empty())
}
