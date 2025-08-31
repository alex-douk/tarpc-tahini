use crate::database::{fetch_user, register_user};
use crate::routes::gen_context;
use rocket::{http::Cookie, route, serde::json::Json as JsonGuard};
// use alohomora::{
//     bbox::BBox,
//     context::Context,
//     rocket::{
//         BBoxCookie, BBoxCookieJar, BBoxJson, JsonResponse, RequestBBoxJson,
//         ResponseBBoxJson, route,
//     },
// };
use rocket::http::CookieJar;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct LoginForm {
    username: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct LoginResponse {
    uuid: Option<String>,
}

#[route(POST, uri = "/login", data = "<data>")]
pub(crate) async fn login(
    cookies: &CookieJar<'_>,
    data: JsonGuard<LoginForm>,
) -> JsonGuard<LoginResponse> {
    // let is_authenticated = cookies.get(name)
    let context = gen_context();
    let uuid = fetch_user(data.username.clone(), context).await;
    match uuid {
        Ok(uuid) => {
            let resp = LoginResponse {
                uuid: Some(uuid.clone()),
            };
            let _ = cookies.add(Cookie::new("user_id", uuid));
            JsonGuard(resp)
        }
        Err(_) => JsonGuard(LoginResponse { uuid: None }),
    }
}

#[route(POST, uri = "/signup", data = "<data>")]
pub(crate) async fn signup(
    cookies: &CookieJar<'_>,
    data: JsonGuard<LoginForm>,
) -> JsonGuard<LoginResponse> {
    let context = gen_context();
    let uuid = register_user(data.username.clone(), context).await;
    match uuid {
        Ok(uuid) => {
            let resp = LoginResponse {
                uuid: Some(uuid.clone()),
            };
            let _ = cookies.add(Cookie::new("user_id", uuid));
            JsonGuard(resp)
        }
        Err(_) => JsonGuard(LoginResponse { uuid: None }),
    }
}
