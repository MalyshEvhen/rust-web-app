use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::user::{UserBmc, UserForLogin};
use crate::model::ModelManager;
use crate::web::{self, remove_token_cookie, Error, Result};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use hmac::digest::typenum::Mod;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;
use tracing_subscriber::field::debug;
use uuid::Uuid;

pub fn routes(mm: ModelManager) -> Router {
  Router::new()
    .route("/api/login", post(api_login_handler))
    .route("/api/logout", post(api_logoff_handler))
    .with_state(mm)
}

async fn api_login_handler(
  State(mm): State<ModelManager>,
  cookies: Cookies,
  Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
  debug!(" {:<12} - api_login_handler", "HANDLER");

  let LoginPayload {
    username,
    pwd: pwd_clear,
  } = payload;
  let root_ctx = Ctx::root_ctx();

  // -- Get the user
  let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
    .await?
    .ok_or(Error::LoginFailUsernameNotFound)?;
  let user_id = user.id;

  // -- Validate the password
  let Some(pwd) = user.pwd else {
    return Err(Error::LoginFailUserHasNoPwd { user_id });
  };

  pwd::validate_password(
    &EncryptContent {
      content: pwd_clear.to_string(),
      salt: user.pwd_salt.to_string(),
    },
    &pwd,
  )
  .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

  // -- Set the web token
  web::set_token_cookie(&cookies, &user.username, &user.token_salt.to_string())?;

  // Create the success body.
  let body = Json(json!({
    "result": {
      "success": true
    }
  }));

  Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  pwd: String,
}

async fn api_logoff_handler(
  cookies: Cookies,
  Json(payload): Json<LogoutPayload>,
) -> Result<Json<Value>> {
  debug!("{:<12} - api_logoff_handler", "HANDLER");
  let should_logoff = payload.logoff;

  if should_logoff {
    remove_token_cookie(&cookies);
  }

  // Create success body
  let body = Json(json!({
    "result": {
      "logged_off": should_logoff
    }
  }));

  Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoutPayload {
  logoff: bool,
}
