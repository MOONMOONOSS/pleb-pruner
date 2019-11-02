use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serenity::{
  client::Client,
  framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
  },
  model::channel::Message,
  prelude::{Context, EventHandler},
};
use std::fs::File;

group!({
  name: "general",
  options: {},
  commands: [
    prune
  ],
});

struct Handler;

impl EventHandler for Handler {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigSchema {
  discord: DiscordConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DiscordConfig {
  guild_id: u64,
  no_prune_ranks: Vec<u64>,
  token: String,
  prune_msg: String,
}

lazy_static!{
  static ref CONFIG: ConfigSchema = get_config();
}

fn get_config() -> ConfigSchema {
  let f = File::open("./config.yaml").unwrap();

  serde_yaml::from_reader(&f).unwrap()
}

fn main() {
  // Bot login
  let mut client: Client =
    Client::new(&CONFIG.discord.token, Handler).expect("Error creating client");

  client.with_framework(
    StandardFramework::new()
      .configure(|c| c.prefix("!"))
      .group(&GENERAL_GROUP),
  );

  // Start listening for events, single shard. Shouldn't need more than one shard
  if let Err(why) = client.start() {
    println!("An error occurred while running the client: {:?}", why);
  }
}


#[command]
fn prune(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
  msg.channel_id.broadcast_typing(&ctx)?;

  msg.reply(
    &ctx,
    "Pruning plebs...",
  )?;

  Ok(())
}
