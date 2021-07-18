




// -----------------
// Development Setup
// ----------------- 
// cargo install cargo-watch && cargo watch -x run



mod handlers;
mod utils;
use crate::utils as botter; //-- or use utils as botter
use std::env;
use dotenv::dotenv;
use teloxide::prelude::*;




#[tokio::main]
async fn main(){
    

    teloxide::enable_logging!();
    log::info!("Starting {} bot...", botter::name);
    dotenv().expect("⚠️ .env file not found");
    let bot = Bot::from_env(); //-- getting API token from .env file



    
    // NOTE - trait Copy is not implemented for Bot struct means we can't assign an instance of it into another variable without moving its ownership
    // NOTE - we cloned the bot instance from Bot struct in order not to move in first api calling
    // NOTE - Bot::clone is relatively cheap, so if you need to share Bot, it's recommended to clone it, instead of wrapping it in Arc<_>.
    botter::test(bot.clone()).await;
    botter::shot(bot.clone()).await;
    botter::burn(bot.clone()).await;




    

    




    
}