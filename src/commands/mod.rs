pub mod register;
pub mod say;
pub mod stats;
use crate::Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub fn commands() -> Vec<poise::Command<Data, Error>> {
  vec![register::register(), stats::stats(), say::say()]
}
