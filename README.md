# TempMail.lol Rust API
Crate API wrapper for [TempMail.lol](https://tempmail.lol) service.

## Installation
Install from [crates.io](https://crates.io). Add the following line to your Cargo.toml file's dependencies section:
```toml
[dependencies]
tempmail-lol = "0.1"
```

## Usage
Library support async and sync functions.

### Sync
```rust
fn main() -> Result<(), tempmail_lol::TempMailError> {
    let inbox = tempmail_lol::create_inbox()?;
    println!("Got random inbox: {:?}", inbox);
}
```

### Async
For async functions you need to use `async` feature flag:
```toml
[dependencies]
tempmail-lol = { version = "0.1", features = ["async"] }
```
And then use any async runtime:
```rust
#[tokio::main]
async fn main() -> Result<(), tempmail_lol::TempMailError> {
    let inbox = tempmail_lol::create_inbox_async().await?;
    println!("Got random inbox: {:?}", inbox);
}
```

**More examples you can find in `examples` folder.**
