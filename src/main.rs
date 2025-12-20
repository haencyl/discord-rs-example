mod commands;
mod events;

use poise::serenity_prelude as serenity;
use std::time::Instant;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::ChronoLocal;

pub struct Data {
  pub start_time: Instant,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  // Custom logging config for human readable output
  tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .with_timer(ChronoLocal::default())
    .with_ansi(true)
    .pretty()
    .init();

  let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN environment variable");

  let intents =
    serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
      on_error: |error| {
        Box::pin(async move {
          match error {
            error => {
              if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling framework error: {e}");
              }
            }
          }
        })
      },
      // Logging for pre- and post-command scenarios
      pre_command: |ctx| {
        Box::pin(async move {
          let guild_id = ctx
            .guild_id()
            .map(|id| id.to_string())
            .unwrap_or_else(|| "None".into());

          info!(
              command = %ctx.command().qualified_name,
              user_id = %ctx.author().id,
              user_name = %ctx.author().name,
              channel_id = %ctx.channel_id(),
              guild_id = %guild_id,
              "Executing command",
          );
        })
      },
      post_command: |ctx| {
        Box::pin(async move {
          let guild_id = ctx
            .guild_id()
            .map(|id| id.to_string())
            .unwrap_or_else(|| "None".into());

          info!(
              command = %ctx.command().qualified_name,
              user_id = %ctx.author().id,
              user_name = %ctx.author().name,
              channel_id = %ctx.channel_id(),
              guild_id = %guild_id,
              "Finished executing command",
          );
        })
      },
      commands: commands::commands(),
      event_handler: |ctx, event, framework, data| {
        Box::pin(events::event_handler(ctx, event, framework, data))
      },
      // Optional prefix commands with *
      prefix_options: poise::PrefixFrameworkOptions {
        prefix: Some("*".into()),
        ..Default::default()
      },
      ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
      Box::pin(async move {
        info!("Registering slash commands globally");
        poise::builtins::register_globally(ctx, &framework.options().commands).await?;
        Ok(Data {
          start_time: Instant::now(),
        })
      })
    })
    .build();

  let client = serenity::ClientBuilder::new(token, intents)
    .framework(framework)
    .await;

  client.unwrap().start().await.unwrap();
}
