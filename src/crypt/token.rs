use std::fmt::Display;
use std::str::FromStr;

use axum::body::HttpBody;

use crate::config;
use crate::crypt::{Error, Result};
use crate::utils::{b64u_decode, b64u_encode};

/// String format: `ident_b64u.exp_b64u.sign_b64u`.
#[derive(Debug)]
pub struct Token {
  pub ident: String,
  pub exp: String,
  pub sign_b64u: String,
}

// FIXME: This should be implemented
impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}.{}.{}",
      b64u_encode(&self.ident),
      b64u_encode(&self.exp),
      self.sign_b64u
    )
  }
}

// FIXME: This should be implemented
impl FromStr for Token {
  type Err = Error;

  fn from_str(token_str: &str) -> std::prelude::v1::Result<Self, Self::Err> {
    let parts: Vec<&str> = token_str.split(".").collect();
    if parts.len() != 3 {
      return Err(Error::TokenInvalidFormat);
    }

    let (ident_b64u, exp_b64u, sign_b64u) = (parts[0], parts[1], parts[2]);

    Ok(Self {
      ident: b64u_decode(ident_b64u).map_err(|_| Error::TokenCannotDecodeIdent)?,
      exp: b64u_decode(exp_b64u).map_err(|_| Error::TokenCannotDecodeExp)?,
      sign_b64u: sign_b64u.to_string(),
    })
  }
}

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
  let config = &config();
  _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_token(orig_token: &Token, salt: &str) -> Result<()> {
  let config = &config();
  _validate_token_sign_and_exp(orig_token, salt, &config.TOKEN_KEY)
}

fn _generate_token(
  indent: &str,
  duration_sec: f64,
  salt: &str,
  key: &[u8],
) -> Result<Token> {
  unimplemented!()
}

fn _validate_token_sign_and_exp(
  orig_token: &Token,
  salt: &str,
  key: &[u8],
) -> Result<()> {
  unimplemented!()
}

fn _token_sign_into_b64u(
  indent: &str,
  exp: &str,
  salt: &str,
  key: &[u8],
) -> Result<String> {
  unimplemented!()
}

#[cfg(test)]
mod tests {
  use super::*;
  use anyhow::{Ok, Result};
  use rand::RngCore;

  #[test]
  fn test_token_display_ok() -> Result<()> {
    let fx_token_str = "ZngtaW5kZW50LTAx.MjAyMy0xMS0xN1QxNTozMDowMFo.some-sign-b64u-encoded";
    let fx_token = Token {
      ident: "fx-indent-01".to_string(),
      exp: "2023-11-17T15:30:00Z".to_string(),
      sign_b64u: "some-sign-b64u-encoded".to_string(),
    };

    assert_eq!(fx_token_str, fx_token.to_string());

    Ok(())
  }

  #[test]
  fn generate_token_from_str_ok() -> Result<()> {
    let fx_token_str = "ZngtaW5kZW50LTAx.MjAyMy0xMS0xN1QxNTozMDowMFo.some-sign-b64u-encoded";
    let fx_token = Token {
      ident: "fx-indent-01".to_string(),
      exp: "2023-11-17T15:30:00Z".to_string(),
      sign_b64u: "some-sign-b64u-encoded".to_string(),
    };

    let token: Token = fx_token_str.parse()?;

    assert_eq!(format!("{fx_token:?}"), format!("{token:?}"));

    Ok(())
  }
}
