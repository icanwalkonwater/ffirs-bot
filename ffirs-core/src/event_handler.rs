use anyhow::Result;
use log::error;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::channel::Message,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        async fn handler(ctx: Context, message: Message) -> Result<()> {
            if message.content.starts_with("!stop") {
                message.channel_id.say(ctx.http, "Stopping...").await?;
            }

            ctx.shard.shutdown_clean();

            Ok(())
        }

        if let Err(err) = handler(ctx, message).await {
            error!("{}", err);
        }
    }
}
