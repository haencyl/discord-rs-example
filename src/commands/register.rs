use super::{Context, Error};
use poise::CreateReply;

/// Reregister all slash commands
#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
  let handle = ctx
    .send(
      CreateReply::default()
        .content("Processing...")
        .ephemeral(true),
    )
    .await?;

  let commands = &ctx.framework().options().commands;
  poise::builtins::register_globally(ctx, commands).await?;

  handle
    .edit(
      ctx,
      CreateReply::default().content(format!("Registered {} commands.", commands.len())),
    )
    .await?;

  Ok(())
}
