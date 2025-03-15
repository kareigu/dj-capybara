use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::gateway::ActivityData;
use serenity::model::{application::Interaction, prelude::*};
use songbird::SerenityInit;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

mod commands;
mod handlers;
mod config;
mod constants;
mod shared;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            handlers::handle_commands(&ctx, command).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let activity = ActivityData::playing("with 🍊");
        ctx.set_activity(Some(activity));

        handlers::register_commands(&ctx, &ready).await;

        info!("{}#{} running", ready.user.name, ready.user.id);
        // initialise tokio::OnceCell
        _ = shared::uptime().await;
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Tracing initialised");
    let config = config::read_config();
    info!("Config read");
    let intents = GatewayIntents::empty()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES;

    info!("Intents: {:?}", intents);

    let latency_map = Arc::new(RwLock::new(shared::ShardLatencyMap::new()));

    let mut client = Client::builder(config.token.clone(), intents)
        .event_handler(Handler)
        .application_id(config.application_id)
        .register_songbird()
        .type_map_insert::<shared::HttpKey>(shared::HttpClient::new())
        .type_map_insert::<shared::ConfigStorage>(Arc::new(config))
        .type_map_insert::<shared::ShardLatencyKey>(latency_map.clone())
        .await
        .expect("Error creating client");

    let manager = client.shard_manager.clone();

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            let runners_lock = manager.runners.lock().await;
            let mut latency_lock = latency_map.write().await;

            for (id, runner) in runners_lock.iter() {
                if let Some(latency) = runner.latency {
                    latency_lock.insert(*id, latency);
                }
            }
        }
    });

    if let Err(e) = client.start_autosharded().await {
        error!("Client error: {:?}", e)
    }
}
