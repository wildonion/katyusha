




// -----------------
// Development Setup
// ----------------- 
// cargo install cargo-watch && cargo watch -x run






mod handlers;
mod utils;
mod commands;
use crate::commands::answer; //-- or use commands::answer
use crate::utils as botter;
use std::env;
use dotenv::dotenv;
use teloxide::prelude::*;




#[tokio::main]
async fn main(){
    





    teloxide::enable_logging!();
    log::info!("Starting {} bot...", botter::name);
    dotenv().expect("⚠️ .env file not found");
    let bot = Bot::from_env().auto_send(); //-- getting API token from .env file - auto_send() will implement Future trait for bot
    teloxide::commands_repl(bot, botter::name, answer).await;




    
}
