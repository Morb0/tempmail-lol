extern crate tempmail_lol;

use std::io::Read;

fn main() -> Result<(), tempmail_lol::TempMailError> {
    let inbox = tempmail_lol::create_inbox()?;
    println!("Got random inbox: {:?}", inbox);

    println!("Send email to '{}' and then press Enter...", inbox.address);
    std::io::stdin().read(&mut [0]).unwrap();

    let emails = tempmail_lol::get_inbox_emails(inbox.token)?;
    println!("Emails in inbox: {:?}", emails);

    Ok(())
}
