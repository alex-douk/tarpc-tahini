use crate::{database::{fetch_user, register_user}, policies::login_uuid::UserIdWebPolicy, routes::gen_context};
use sesame::{
    pcon::PCon,
    context::Context,
};


use sesame_rocket::rocket::{
        PConCookie, PConCookieJar, PConJson, JsonResponse, RequestPConJson,
        ResponsePConJson, route,
};
use core_tahini_utils::policies::UsernamePolicy;
use std::collections::HashMap;

#[derive(Clone, RequestPConJson)]
pub struct LoginForm {
    username: PCon<String, UsernamePolicy>,
}

#[derive(Clone, ResponsePConJson)]
pub struct LoginResponse {
    uuid: Option<PCon<String, UserIdWebPolicy>>,
}

#[route(POST, "/login", data = "<data>")]
pub(crate) async fn login(
    cookies: PConCookieJar<'_, '_>,
    data: PConJson<LoginForm>,
) -> sesame_rocket::rocket::JsonResponse<LoginResponse, ()> {
    // let is_authenticated = cookies.get(name)
    let context = gen_context();
    let uuid = fetch_user(data.username.clone(), context).await;
    match uuid {
        Ok(uuid) => {
            let resp = LoginResponse {
                uuid: Some(uuid.clone()),
            };
            let _ = cookies.add(PConCookie::new("user_id", uuid), Context::<()>::empty());
            JsonResponse(resp, Context::empty())
        }
        Err(_e) => JsonResponse(LoginResponse { uuid: None }, Context::empty()),
    }
}

#[route(POST, "/signup", data = "<data>")]
pub(crate) async fn signup(
    cookies: PConCookieJar<'_, '_>,
    data: PConJson<LoginForm>,
) -> sesame_rocket::rocket::JsonResponse<LoginResponse, ()> {
    let context = gen_context();
    let uuid = register_user(data.username.clone(), context).await;
    match uuid {
        Ok(uuid) => {
            let resp = LoginResponse {
                uuid: Some(uuid.clone()),
            };
            let _ = cookies.add(PConCookie::new("user_id", uuid), Context::<()>::empty());
            JsonResponse(resp, Context::empty())
        }
        Err(_e) => JsonResponse(LoginResponse { uuid: None }, Context::empty()),
    }
}
