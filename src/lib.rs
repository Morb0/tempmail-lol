use serde::Deserialize;
use thiserror::Error;

mod blocking_req;
pub use blocking_req::*;

#[cfg(feature = "async")]
mod async_req;
#[cfg(feature = "async")]
pub use async_req::*;

#[derive(Debug, Deserialize)]
pub struct Inbox {
    pub address: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DomainInboxResponse {
    Success(Inbox),
    #[allow(dead_code)]
    Error { error: String }
}

#[derive(Debug, Deserialize)]
pub struct Email {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
    pub html: Option<String>,
    pub date: i64,
    pub ip: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum EmailsResponse {
    Success{ email: Vec<Email> },
    #[allow(dead_code)]
    Error { token: String }
}

#[derive(Debug, Error)]
pub enum TempMailError {
    #[error("request failed")]
    RequestError(#[from] reqwest::Error),
    #[error("invalid domain")]
    InvalidDomain,
    #[error("invalid token")]
    InvalidToken,
}

const BASE_URL: &str = "https://api.tempmail.lol";

