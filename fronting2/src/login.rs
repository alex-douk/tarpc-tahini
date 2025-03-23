use crate::database::{fetch_user, register_user};
use alohomora::{
    bbox::BBox,
    context::Context,
    rocket::{
        BBoxCookie, BBoxCookieJar, BBoxJson, BBoxRedirect, JsonResponse, RequestBBoxJson,
        ResponseBBoxJson, route,
    },
};
use services_utils::policies::shared_policies::UsernamePolicy;
use std::collections::HashMap;

#[derive(Clone, RequestBBoxJson)]
pub struct LoginForm {
    username: BBox<String, UsernamePolicy>,
}

#[derive(Clone, ResponseBBoxJson)]
pub struct LoginResponse {
    uuid: Option<BBox<String, UsernamePolicy>>,
}

#[route(POST, "/login", data = "<data>")]
pub(crate) async fn login(
    cookies: BBoxCookieJar<'_, '_>,
    data: BBoxJson<LoginForm>,
) -> alohomora::rocket::JsonResponse<LoginResponse, ()> {
    // let is_authenticated = cookies.get(name)
    let uuid = fetch_user(data.username.clone()).await;
    match uuid {
        Ok(uuid) => {
            let resp = LoginResponse {
                uuid: Some(uuid.clone()),
            };
            let _ = cookies.add(BBoxCookie::new("user_id", uuid), Context::<()>::empty());
            JsonResponse(resp, Context::empty())
        }
        Err(e) => JsonResponse(LoginResponse { uuid: None }, Context::empty()),
    }
}

#[route(POST, "/signup", data = "<data>")]
pub(crate) async fn signup(
    cookies: BBoxCookieJar<'_, '_>,
    data: BBoxJson<LoginForm>,
) -> alohomora::rocket::JsonResponse<LoginResponse, ()> {
    let uuid = register_user(data.username.clone()).await;
    match uuid {
        Ok(uuid) => {
            let resp = LoginResponse {
                uuid: Some(uuid.clone()),
            };
            let _ = cookies.add(BBoxCookie::new("user_id", uuid), Context::<()>::empty());
            JsonResponse(resp, Context::empty())
        }
        Err(e) => JsonResponse(LoginResponse { uuid: None }, Context::empty()),
    }
}
