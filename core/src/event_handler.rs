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
        #[inline(always)]
        async fn consumer(ctx: Context, message: Message) -> Result<()> {
            if message.content == "!stop" {
                message.channel_id.say(ctx.http, "Stopping...").await?;
            }

            ctx.shard.shutdown_clean();

            Ok(())
        };

        if let Err(err) = consumer(ctx, message).await {
            error!("{}", err);
        }
    }
}
