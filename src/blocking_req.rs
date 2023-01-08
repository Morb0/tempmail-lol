use crate::*;

pub fn create_inbox() -> Result<Inbox, TempMailError> {
    let url = format!("{BASE_URL}/generate");
    let data: Inbox = reqwest::blocking::get(url)?.json()?;
    Ok(data)
}

pub fn create_rush_inbox() -> Result<Inbox, TempMailError> {
    let url = format!("{BASE_URL}/generate/rush");
    let data: Inbox = reqwest::blocking::get(url)?.json()?;
    Ok(data)
}

pub fn create_domain_inbox(domain: String) -> Result<Inbox, TempMailError> {
    let url = format!("{BASE_URL}/generate/{domain}");
    let data: DomainInboxResponse = reqwest::blocking::get(url)?.json()?;
    match data {
        DomainInboxResponse::Error { .. } => Err(TempMailError::InvalidDomain),
        DomainInboxResponse::Success { address, token } => Ok(Inbox { address, token }),
    }
}

pub fn get_inbox_emails(token: String) -> Result<Vec<Email>, TempMailError> {
    let url = format!("{BASE_URL}/auth/{token}");
    let data: EmailsResponse = reqwest::blocking::get(url)?.json()?;
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

pub fn get_custom_inbox_emails(domain: String, key: String) -> Result<Vec<Email>, TempMailError> {
    let url = format!("{BASE_URL}/custom/{key}/{domain}");
    let data: EmailsResponse = reqwest::blocking::get(url)?.json()?;
    match data.email {
        Some(data) => Ok(data),
        None => Ok(Vec::new()),
    }
}
