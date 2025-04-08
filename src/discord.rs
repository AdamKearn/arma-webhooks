use crate::{get_config, RUNTIME};
use discord_webhook::client::WebhookClient;

pub fn send(hook_name: String, message_body: String) -> String {
    let config = get_config();

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
                    if let Some(content) = &hook_details.content {
                        message.content(&content.replace("***", &message_body));
                    } else {
                        message.content(&message_body);
                    }

                    message
                })
                .await
                .unwrap();
        });

        format!("success")
    } else {
        format!("failed")
    }
}
