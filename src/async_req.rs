use serde::de::DeserializeOwned;
use crate::*;

pub async fn create_inbox_async() -> Result<Inbox, TempMailError> {
    let url = format!("{BASE_URL}/generate");
    let data: Inbox = make_request(url).await?;
    Ok(data)
}

pub async fn create_rush_inbox_async() -> Result<Inbox, TempMailError> {
    let url = format!("{BASE_URL}/generate/rush");
    let data: Inobx = make_request(url).await?;
    Ok(data)
}

pub async fn create_domain_inbox_async(domain: String) -> Result<Inbox, TempMailError> {
    let url = format!("{BASE_URL}/generate/{domain}");
    let data: DomainInboxResponse = make_request(url).await?;
    match data {
        DomainInboxResponse::Error => Err(TempMailError::InvalidDomain),
        DomainInboxResponse::Success => data,
    }
}

pub async fn get_inbox_emails_async(token: String) -> Result<Vec<Email>, TempMailError> {
    let url = format!("{BASE_URL}/auth/{token}");
    let data: EmailsResponse = make_request(url).await?;
    if let Some(data) = data.email {
        return Ok(data);
    }

    if let Some(msg) = data.token {
        if msg == "invalid" {
            return Err(TempMailError::InvalidDomain)
        }
    }

    Ok(Vec::new())
}

pub async fn get_custom_inbox_emails_async(domain: String, key: String) -> Result<Vec<Email>, TempMailError> {
    let url = format!("{BASE_URL}/custom/{key}/{domain}");
    let data: EmailsResponse = make_request(url).await?;
    match data.email {
        Some(data) => Ok(data),
        None => Ok(Vec::new()),
    }
}

async fn make_request<T: DeserializeOwned>(url: String) -> Result<T, TempMailError> {
    Ok(reqwest::get(url).await?.json::<T>().await?)
}
