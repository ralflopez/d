use axum::extract::State;
use tower_cookies::{Cookie, Cookies};

use crate::model::ModelManager;

pub struct LoginCredentials {
    username: String,
    password: String,
}

pub const AUTH_TOKEN: &str = "auth-token";

async fn login_handler(State(_mm): State<ModelManager>, cookies: Cookies) {
    let token = "t";
    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);
}
