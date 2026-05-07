use super::{Context, Error};
use poise::CreateReply;

const SECS_PER_DAY: u64 = 86_400;
const SECS_PER_HOUR: u64 = 3_600;
const SECS_PER_MINUTE: u64 = 60;

/// List bot statistics
#[poise::command(slash_command)]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
  let latency = ctx.ping().await;
  let uptime = ctx.data().start_time.elapsed();

  let days = uptime.as_secs() / SECS_PER_DAY;
  let hours = (uptime.as_secs() % SECS_PER_DAY) / SECS_PER_HOUR;
  let minutes = (uptime.as_secs() % SECS_PER_HOUR) / SECS_PER_MINUTE;
  let seconds = uptime.as_secs() % SECS_PER_MINUTE;

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
