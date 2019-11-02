use lazy_static::lazy_static;
use math::round;
use serde::{Deserialize, Serialize};
use serenity::{
  client::Client,
  framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
  },
  model::{
    channel::Message,
    guild::Member,
    id::{
      UserId,
      RoleId,
    },
  },
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
  // Get total members
  // We can only iterate through guild members in 1,000 member chunks
  let total_members: u64 = msg.guild(&ctx)
    .unwrap()
    .read()
    .member_count;
  let mut last_member_id: UserId = UserId(0);

  println!("Total Members: {}", total_members);

  'batch_handler: for x in 0..(round::ceil((total_members / 1000) as f64, 0) + 1.0) as u64 {
    println!("Retrieving member batch {}...", x);
    let batch_members: Vec<Member>;

    // Get member batch
    match last_member_id.as_u64() {
      // If first iteration
      0 => {
        batch_members = msg.guild(&ctx)
          .unwrap()
          .read()
          .members(&ctx, Some(1000), None)
          .unwrap();
      },
      // Subsequent iterations
      _ => {
        batch_members = msg.guild(&ctx)
          .unwrap()
          .read()
          .members(&ctx, Some(1000), Some(last_member_id))
          .unwrap();
      },
    };
    
    'member_handler: for member in &batch_members {
      let mut is_pleb: bool = true;
      // Checks user roles against a config-backed list of non-pleb roles
      'role_handler: for role in &CONFIG.discord.no_prune_ranks {
        if member.roles.contains(&RoleId(*role)) {
          is_pleb = false;
          break 'role_handler
        }
      }
      if is_pleb {
        println!("PLEB LOCATED: {}", member.user_id().as_u64())
      }
      last_member_id = member.user_id()
    }
  };
  
  Ok(())
}
