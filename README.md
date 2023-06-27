# Rust Echo Bot for Telegram
This is a simple Echo Bot implemented in Rust using the teloxide library, which allows you to create Telegram bots easily.

## Prerequisites
Before running the bot, make sure you have the following:

- Rust programming language installed (you can get it from https://www.rust-lang.org/)
- Telegram Bot token: You will need a bot token to authenticate with the Telegram Bot API. If you don't have one, you can create a new bot and obtain the token by following the Telegram Bot documentation.

## Installation

1. Create a new Rust project:
```bash
cargo new your-repository
```
2. Open the Cargo.toml file in the project root directory.

3. Replace the contents of Cargo.toml with the following:
```toml
[package]
name = "your-repository"
version = "0.1.0"
edition = "2021"

[dependencies]
teloxide = { version = "0.12", features = ["macros"] }
tokio = { version =  "1.28", features = ["rt-multi-thread", "macros"] }
```

4. Change to the project directory:
```bash
cd your-repository
```

## Code

Open the `src/main.rs` file and replace its contents with the following code:

```rust
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    let bot = Bot::new("YOUR_TOKEN_HERE");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_message(msg.chat.id, msg.text().unwrap_or_default()).await?;
        Ok(())
    })
    .await;
}
```

## Configuration
1. Open the `src/main.rs` file in the project root directory.
   
2. Locate the line that says `let bot = Bot::new("YOUR_TOKEN_HERE");`.
   
3. Replace `"YOUR_TOKEN_HERE"` with your actual Telegram Bot token obtained earlier.

## Usage
1. Run the bot using the following command:

```bash

cargo run --release

```

2.Start a conversation with your bot on Telegram.

3.Send a message to the bot, and it will reply with the same message.

## Contributing
Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or submit a pull request on GitHub.

## License
This project is licensed under the MIT License.
