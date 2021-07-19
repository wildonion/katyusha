





use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommand};
use crate::utils as botter; //-- or use utils as botter



#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Attack with one the following commands:")]
pub enum Command {
    #[command(description = "Take Screenshot")]
    Shot,
    #[command(description = "Burn CPU.")]
    Burn,
    #[command(description = "Roll Dice.")]
    Roll,
}




pub async fn answer(cx: UpdateWithCx<AutoSend<Bot>, Message>, command: Command) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        
        Command::Shot => botter::shot(cx).await,
        Command::Burn => botter::burn(cx).await,
        Command::Roll => botter::roll(cx).await,
    };
    
    Ok(())
}