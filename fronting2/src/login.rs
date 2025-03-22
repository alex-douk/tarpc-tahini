use crate::database::fetch_or_insert_user;
use alohomora::{
    bbox::BBox,
    context::Context,
    rocket::{BBoxJson, JsonResponse, RequestBBoxJson, ResponseBBoxJson, route},
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
    data: BBoxJson<LoginForm>,
) -> alohomora::rocket::JsonResponse<LoginResponse, ()> {
    let resp = LoginResponse {
        uuid: fetch_or_insert_user(data.username.clone()).await,
    };
    JsonResponse(resp, Context::empty())
}
