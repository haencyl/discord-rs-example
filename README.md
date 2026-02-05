# discord-rs-example

An example Discord Bot written in [Rust] with a custom slash- & optional prefix command and event handler.
It uses [Serenity] & [Poise].

[Rust]: https://rust-lang.org/
[Serenity]: https://github.com/serenity-rs/serenity
[Poise]: https://github.com/serenity-rs/poise

## Running the Bot

To run the bot, you must provide your credentials via environment variables. Example usage:

```bash
DISCORD_TOKEN="your_token_here" \
RUST_LOG=info \
cargo run --release
```

## Environment Variables

- DISCORD_TOKEN: The secret token from the [Discord Developer Portal](https://discord.com/developers/applications).
- RUST_LOG: Controls the logging verbosity. Setting it to info provides standard execution updates, while debug shows more detailed event data.

## Commands

- `/stats` - List Bot statistics like uptime and ping.
- `/say` - Let the bot send a message.

## Events

- `ready` - Triggers when the bot is online, sets custom status.
- `message` - Triggers when the bot gets mentioned and responds with "Hello!".
