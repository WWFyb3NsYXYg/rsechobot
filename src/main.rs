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
