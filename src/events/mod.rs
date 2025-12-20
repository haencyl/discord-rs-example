pub mod ready;

use crate::{Data, commands::Error};
use poise::serenity_prelude as serenity;

pub async fn event_handler(
  ctx: &serenity::Context,
  event: &serenity::FullEvent,
  _framework: poise::FrameworkContext<'_, Data, Error>,
  _data: &Data,
) -> Result<(), Error> {
  match event {
    serenity::FullEvent::Ready { data_about_bot } => {
      ready::handle(ctx, data_about_bot).await;
    }
    _ => {}
  }
  Ok(())
}
