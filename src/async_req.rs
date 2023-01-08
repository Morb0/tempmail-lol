use serde::de::DeserializeOwned;
use crate::*;

/// Create new inbox with random name and domain
pub async fn create_inbox_async() -> Result<Inbox, TempMailError> {
    let url = format!("{BASE_URL}/generate");
    let data: Inbox = make_request(url).await?;
    Ok(data)
}

/// Create new inbox with random name and domain in n [rush mode](https://tempmail.lol/news/2022/08/03/introducing-alternative-emails-for-tempmail/)
pub async fn create_rush_inbox_async() -> Result<Inbox, TempMailError> {
    let url = format!("{BASE_URL}/generate/rush");
    let data: Inbox = make_request(url).await?;
    Ok(data)
}

/// Create new inbox with random name and selected domain
pub async fn create_domain_inbox_async(domain: String) -> Result<Inbox, TempMailError> {
    let trimmed = domain.trim();
    if trimmed.is_empty() {
        return Err(TempMailError::InvalidDomain);
    }
    let url = format!("{BASE_URL}/generate/{trimmed}");
    let data: DomainInboxResponse = make_request(url).await?;
    match data {
        DomainInboxResponse::Error { .. } => Err(TempMailError::InvalidDomain),
        DomainInboxResponse::Success(inbox) => Ok(inbox),
    }
}

/// Retrieve messages from inbox by token
pub async fn get_inbox_emails_async(token: String) -> Result<Vec<Email>, TempMailError> {
    let url = format!("{BASE_URL}/auth/{token}");
    let data: EmailsResponse = make_request(url).await?;
    match data {
        EmailsResponse::Error { .. } => Err(TempMailError::InvalidToken),
        EmailsResponse::Success { email } => Ok(email),
    }
}

/// Retrieve all messages from private domain inbox
pub async fn get_custom_inbox_emails_async(domain: String, key: String) -> Result<Vec<Email>, TempMailError> {
    let url = format!("{BASE_URL}/custom/{key}/{domain}");
    let data: EmailsResponse = make_request(url).await?;
    match data {
        EmailsResponse::Success { email } => Ok(email),
        _ => Ok(Vec::new()),
    }
}

async fn make_request<T: DeserializeOwned>(url: String) -> Result<T, TempMailError> {
    Ok(reqwest::get(url).await?.json::<T>().await?)
}
