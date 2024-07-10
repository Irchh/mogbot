use serenity::all::{CreateCommand, ResolvedOption};

pub fn run(_: &[ResolvedOption]) -> String {
    "Pong!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
