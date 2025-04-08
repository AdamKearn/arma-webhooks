use crate::{get_config, RUNTIME};
use reqwest::Client;

pub fn get(hook_name: String, message_body: String) -> String {
    let config = get_config();

    if let Some(hook_details) = config.rest.get(&hook_name) {
        RUNTIME.block_on(async move {
            let client = Client::new();
            let payload = serde_json::json!({
                "content": message_body
            });

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
