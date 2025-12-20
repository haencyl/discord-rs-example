use super::{Context, Error};
use poise::CreateReply;

/// List bot statistics
#[poise::command(slash_command)]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
  let latency = ctx.ping().await;
  let uptime = ctx.data().start_time.elapsed();

  let days = uptime.as_secs() / 86_400;
  let hours = (uptime.as_secs() % 86_400) / 3_600;
  let minutes = (uptime.as_secs() % 3_600) / 60;
  let seconds = uptime.as_secs() % 60;

  let content = format!(
    "Latency: `{}ms`\nUptime: `{}d {}h {}m {}s`",
    latency.as_millis(),
    days,
    hours,
    minutes,
    seconds
  );

  ctx
    .send(CreateReply::default().content(content).ephemeral(true))
    .await?;

  Ok(())
}
