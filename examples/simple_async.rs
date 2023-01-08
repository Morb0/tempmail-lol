extern crate tempmail_lol;

use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), tempmail_lol::TempMailError> {
    let inbox = tempmail_lol::create_inbox_async().await?;
    println!("Got random inbox: {:?}", inbox);

    println!("Send email to '{}' and then press Enter...", inbox.address);
    std::io::stdin().read(&mut [0]).unwrap();

    let emails = tempmail_lol::get_inbox_emails_async(inbox.token).await?;
    println!("Emails in inbox: {:?}", emails);

    Ok(())
}
