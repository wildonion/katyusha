







use cmd;
use std::env;
use dotenv::dotenv;
use teloxide::prelude::*;


#[tokio::main]
async fn main(){
    

    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");
    dotenv().expect("⚠️ .env file not found");
    let bot = Bot::from_env().auto_send();
    


    // TODO - implement /bc and /sc commands
    // ...

    

    teloxide::repl(bot, |message| async move {
        message.answer_dice().await?;
        respond(())
    })
    .await;




    
}