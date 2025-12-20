use super::{Context, Error};
use poise::CreateReply;

/// Let the bot send a message
#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn say(
  ctx: Context<'_>,
  #[description = "Message to say"] message: String,
) -> Result<(), Error> {
  ctx
    .send(
      CreateReply::default()
        .content("Message sent successfully")
        .ephemeral(true),
    )
    .await?;
  ctx.channel_id().say(ctx, message).await?;

  Ok(())
}
