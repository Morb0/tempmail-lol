use crate::*;

/// Create new inbox with random name and domain
pub fn create_inbox() -> Result<Inbox, TempMailError> {
    let url = format!("{BASE_URL}/generate");
    let data: Inbox = reqwest::blocking::get(url)?.json()?;
    Ok(data)
}

/// Create new inbox with random name and domain in n [rush mode](https://tempmail.lol/news/2022/08/03/introducing-alternative-emails-for-tempmail/)
pub fn create_rush_inbox() -> Result<Inbox, TempMailError> {
    let url = format!("{BASE_URL}/generate/rush");
    let data: Inbox = reqwest::blocking::get(url)?.json()?;
    Ok(data)
}

/// Create new inbox with random name and selected public domain
pub fn create_domain_inbox(domain: String) -> Result<Inbox, TempMailError> {
    let trimmed = domain.trim();
    if trimmed.is_empty() {
        return Err(TempMailError::InvalidDomain);
    }
    let url = format!("{BASE_URL}/generate/{trimmed}");
    let data: DomainInboxResponse = reqwest::blocking::get(url)?.json()?;
    match data {
        DomainInboxResponse::Error { .. } => Err(TempMailError::InvalidDomain),
        DomainInboxResponse::Success(inbox) => Ok(inbox),
    }
}

/// Retrieve messages from inbox by token
pub fn get_inbox_emails(token: String) -> Result<Vec<Email>, TempMailError> {
    let url = format!("{BASE_URL}/auth/{token}");
    let data: EmailsResponse = reqwest::blocking::get(url)?.json()?;
    match data {
        EmailsResponse::Error { .. } => Err(TempMailError::InvalidToken),
        EmailsResponse::Success { email } => Ok(email),
    }
}

/// Retrieve all messages from private domain inbox
pub fn get_custom_inbox_emails(domain: String, key: String) -> Result<Vec<Email>, TempMailError> {
    let url = format!("{BASE_URL}/custom/{key}/{domain}");
    let data: EmailsResponse = reqwest::blocking::get(url)?.json()?;
    match data {
        EmailsResponse::Success { email } => Ok(email),
        _ => Ok(Vec::new()),
    }
}
