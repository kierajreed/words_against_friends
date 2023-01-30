use futures::stream::StreamExt;
use std::env;
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard};

mod discord;
mod game;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let prefix = String::from("w::");
  let token = env::var("DISCORD_TOKEN_WAF")?;
  let intents = Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;
  let event_types = EventTypeFlags::MESSAGE_CREATE;

  let (shard, mut events) = Shard::builder(token.clone(), intents).event_types(event_types).build();
  shard.start().await?;

  let mut bot = discord::DiscordBot::new(prefix, token);

  while let Some(event) = events.next().await {
    match event {
      Event::MessageCreate(message) => {
        bot.handle_message(message.0).await;
      }
      _ => {}
    }
  }

  Ok(())
}
