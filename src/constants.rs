use chrono::{DateTime, Local, TimeDelta};
pub use reqwest::Client as HttpClient;
use serenity::model::{Colour, id::ShardId};
use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration};
use tokio::sync::{OnceCell, RwLock};

pub enum ErrorCodes {
    ConfigFileError = 10,
}

pub fn placeholder_img() -> String {
    "https://karei.dev/files/capybara-default.jpg".to_string()
}

pub const EMBED_COLOUR: Colour = Colour::from_rgb(232, 12, 116);
pub const COMMIT_URL: &str = "https://git.sr.ht/~kareigu/capybotbara/commit/";

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

pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUST_VERSION: &str = env!("RUSTC_SEMVER");
pub const LLVM_VERSION: &str = env!("RUSTC_LLVM_VERSION");
pub const HOST_TRIPLE: &str = env!("RUSTC_HOST_TRIPLE");
pub const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
pub const GIT_DESC: &str = env!("GIT_COMMIT");
