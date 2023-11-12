use crate::RUNTIME;
use discord_webhook::client::WebhookClient;

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct DiscordConfig {
    endpoint: String,
    username: Option<String>,
    avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    discord: HashMap<String, DiscordConfig>,
}

pub fn send(hook_name: String, message_body: String) -> String {
    let config_content = fs::read_to_string("/home/reload/arma-server/@ArmaWebhooks/config.yaml")
        .expect("Unable to read config.yaml file");
    let config: Config = serde_yaml::from_str(&config_content).expect("Invalid YAML");

    if let Some(hook_details) = config.discord.get(&hook_name) {
        RUNTIME.block_on(async move {
            let client: WebhookClient = WebhookClient::new(&hook_details.endpoint);
            client
                .send(|message| {
                    if let Some(username) = &hook_details.username {
                        message.username(&username);
                    }
                    if let Some(avatar_url) = &hook_details.avatar_url {
                        message.avatar_url(&avatar_url);
                    }

                    message.embed(|embed| {
                        if let Some(username) = &hook_details.username {
                            embed.title(&username);
                        }
                        embed.description(&message_body)
                    })
                })
                .await
                .unwrap();
        });

        format!("success")
    } else {
        format!("failed")
    }
}
