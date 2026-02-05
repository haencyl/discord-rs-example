use ::serenity::all::{ActivityData, OnlineStatus};
use poise::serenity_prelude as serenity;
use tracing::info;

pub async fn handle(ctx: &serenity::Context, ready: &serenity::Ready) {
  info!(
    bot_id = %ready.user.id,
    bot_name = %ready.user.name,
    "Bot is now online",
  );

  ctx.set_presence(
    Some(ActivityData::custom("Hello World!")),
    OnlineStatus::Online,
  );
}
