use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(login))
}

async fn login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // let username = payload.username.clone();
    // let password = payload.password.clone();
    // let mut user = crate::user::User::new(username, password);
    // user.login().await?;
    // Ok(Json(serde_json::json!({"status": "ok"})))

    if payload.username != "admin" || payload.password != "admin" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(crate::web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result":{
            "success":true,
        }
    }));

    Ok(body)
}
