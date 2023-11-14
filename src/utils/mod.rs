mod error;

pub use self::error::{Error, Result};

use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

pub fn now_utc() -> OffsetDateTime {
  OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
  time.format(&Rfc3339).unwrap() // TODO: need to check if safe
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
  let new_time = now_utc() + Duration::seconds_f64(sec);
  format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
  OffsetDateTime::parse(moment, &Rfc3339)
    .map_err(|_| Error::DateFailParse(moment.to_string()))
}

pub fn b64u_encode(value: &str) -> String {
  base64_url::encode(value.as_bytes())
}

pub fn b64u_decode(value: &str) -> Result<String> {
  base64_url::decode(value)
    .ok()
    .and_then(|v| String::from_utf8(v).ok())
    .ok_or(Error::FailToB64uDecode)
}
