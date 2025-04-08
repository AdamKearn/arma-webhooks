use arma_rs::{arma, Extension, Group};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs, path::Path};

mod discord;

static RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Unable to start the runtime.")
});

#[derive(Debug, Deserialize)]
struct DiscordConfig {
    endpoint: String,
    username: Option<String>,
    avatar_url: Option<String>,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    discord: std::collections::HashMap<String, DiscordConfig>,
}

fn get_config() -> Config {
    let config_path = Path::new(&std::env::current_dir().unwrap()).join("@arma_webhooks").join("config.yaml");
    let config_content = fs::read_to_string(config_path)
        .expect("Unable to read config.yaml file");
    let config: Config = serde_yaml::from_str(&config_content).expect("Invalid YAML");

    config
}

#[arma]
fn init() -> Extension {
    let ext = Extension::build().command("discord", discord::send).group("discord", Group::new().command("send", discord::send)); 
    ext.finish()
}
