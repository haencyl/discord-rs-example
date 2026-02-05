use poise::serenity_prelude as serenity;
use serenity::model::channel::Message;
use tracing::info;

pub async fn handle(ctx: &serenity::Context, message: &Message) {
  if !message.author.bot {
    info!(
      "Received message from {}: '{}'",
      message.author.name, message.content
    );

    if message.mentions_me(&ctx).await.unwrap_or(false) {
      match message.channel_id.say(ctx, "Hello!").await {
        Ok(sent_message) => {
          info!(
            "Sent reply: '{}' to channel {}",
            sent_message.content, sent_message.channel_id
          );
        }
        Err(e) => {
          info!("Failed to reply to message: {:?}", e);
        }
      }
    }
  }
}
