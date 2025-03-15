use serenity::model::Colour;

pub fn placeholder_img() -> String {
    "https://karei.dev/files/capybara-default.jpg".to_string()
}

pub const EMBED_COLOUR: Colour = Colour::from_rgb(232, 12, 116);
pub const COMMIT_URL: &str = "https://git.sr.ht/~kareigu/capybotbara/commit/";

pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUST_VERSION: &str = env!("RUSTC_SEMVER");
pub const LLVM_VERSION: &str = env!("RUSTC_LLVM_VERSION");
pub const HOST_TRIPLE: &str = env!("RUSTC_HOST_TRIPLE");
pub const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
pub const GIT_DESC: &str = env!("GIT_COMMIT");
