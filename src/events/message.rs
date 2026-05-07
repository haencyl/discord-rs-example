use poise::serenity_prelude as serenity;
use serenity::model::channel::Message;
use tracing::{info, warn};

pub async fn handle(ctx: &serenity::Context, message: &Message) {
  if message.author.bot {
    return;
  }

  info!(
    "Received message from {}: '{}'",
    message.author.name, message.content
  );

  match message.mentions_me(&ctx).await {
    Ok(true) => match message.channel_id.say(ctx, "Hello!").await {
      Ok(sent_message) => {
        info!(
          "Sent reply: '{}' to channel {}",
          sent_message.content, sent_message.channel_id
        );
      }
      Err(e) => {
        warn!("Failed to reply to message: {:?}", e);
      }
    },
    Ok(false) => {}
    Err(e) => {
      warn!("Failed to check if bot was mentioned: {e}");
    }
  }
}
