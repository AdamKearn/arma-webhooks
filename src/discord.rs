use crate::RUNTIME;
use arma_rs::Group;
use discord_webhook::client::WebhookClient;

pub fn group() -> Group {
    Group::new().command("send", send)
}

fn send(message_body: String) -> String {
    RUNTIME.block_on(async move {
        let client: WebhookClient = WebhookClient::new("https://discord.com/api/webhooks/1168673937731891250/y5Wv3_pHoLHebrBP8lLVbonAX4z3nn_XyVbIKbxsEw3SUYyMjrZHXbM8ByN6_BF1xX6H");
        let result = client
            .send(|message| message.embed(|embed| embed.title("Webhook").description(&message_body)))
            .await;

        print!("{:?}", result);
    });

    format!("success")
}
