use crate::{get_config, RUNTIME};
use reqwest::Client;

pub fn send(hook_name: String, message_body: String) -> String {
    let config = get_config();

    if let Some(hook_details) = config.discord.get(&hook_name) {
        RUNTIME.block_on(async move {
            let client = Client::new();
            let mut payload = serde_json::json!({
                "content": message_body
            });

            if let Some(username) = &hook_details.username {
                payload["username"] = serde_json::Value::String(username.clone());
            }
            if let Some(avatar_url) = &hook_details.avatar_url {
                payload["avatar_url"] = serde_json::Value::String(avatar_url.clone());
            }
            if let Some(content) = &hook_details.content {
                payload["content"] = serde_json::Value::String(content.replace("***", &message_body));
            }

            let response = client
                .post(&hook_details.endpoint)
                .json(&payload)
                .send()
                .await;

            if response.is_ok() {
                format!("success")
            } else {
                format!("failed: {}", response.unwrap_err())
            }
        })
    } else {
        format!("unable to find hook with name: {}", hook_name)
    }
}
