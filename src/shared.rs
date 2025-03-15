use crate::config::Config;
use chrono::{DateTime, Local, TimeDelta};
pub use reqwest::Client as HttpClient;
use serenity::{model::id::ShardId, prelude::TypeMapKey};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::{OnceCell, RwLock};

pub struct ConfigStorage;

impl TypeMapKey for ConfigStorage {
    type Value = Arc<Config>;
}

pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

pub struct ShardLatencyKey;

pub type ShardLatencyMap = HashMap<ShardId, Duration>;

impl TypeMapKey for ShardLatencyKey {
    type Value = Arc<RwLock<ShardLatencyMap>>;
}

static UPTIME: OnceCell<DateTime<Local>> = OnceCell::const_new();
pub async fn uptime() -> (TimeDelta, &'static DateTime<Local>) {
    let start = UPTIME.get_or_init(|| async { Local::now() }).await;
    (Local::now() - start, start)
}
