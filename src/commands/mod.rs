pub mod register;
pub mod say;
pub mod stats;
use crate::{Data, Error};

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub fn commands() -> Vec<poise::Command<Data, Error>> {
  vec![register::register(), stats::stats(), say::say()]
}
