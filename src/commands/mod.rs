pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn mogping(ctx: Context<'_>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ctx.say("MogPong!").await?;
    Ok(())
}
