extern crate tempmail_lol;

use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), tempmail_lol::TempMailError> {
    println!("Enter custom email domain (Example: lamasticots.com):");
    let mut domain = String::new();
    std::io::stdin().read_line(&mut domain).unwrap();

    let inbox = tempmail_lol::create_domain_inbox_async(domain).await?;
    println!("Got random inbox: {:?}", inbox);

    println!("Send email to '{}' and then press Enter...", inbox.address);
    std::io::stdin().read(&mut [0]).unwrap();

    let emails = tempmail_lol::get_inbox_emails_async(inbox.token).await?;
    println!("Emails in inbox: {:?}", emails);

    Ok(())
}
