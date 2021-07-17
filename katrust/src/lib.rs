



// https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa
// https://danielkeep.github.io/tlborm/book/
// https://cetra3.github.io/blog/implementing-a-jobq/
// https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/
// https://docs.rs/tokio/1.7.1/tokio/sync/index.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://blog.logrocket.com/procedural-macros-in-rust/
// https://github.com/cksac/dataloader-rs
// http://gradebot.org/doc/ipur/trait.html
// https://doc.rust-lang.org/std/sync/struct.Arc.html
// https://doc.rust-lang.org/std/sync/struct.Mutex.html
// http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/procedural-macros.html
// https://dev.to/5422m4n/rust-macro-rules-in-practice-40ne
// https://github.com/teloxide/teloxide/tree/dev/examples
// https://github.com/TeXitoi/structopt
// https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html




// TODO - custom derive macro or proc_macro for my own traits using trait scope orphan rule, closures
//        Rc, RefCell, Arc, Mutex, RwLck for thread safe coding
//        asynchronous task handler (async move) and multithreaded pattern using thread pool and mpsc job queue channel protocol based on futures to move data between threads
//        move asynchronous tasks between tokio::spawn() threads using mpsc job queue channel with tokio::sync::mpsc methods (channel and unbounded_channel)
//        derive attributes and proc macros and generators





// can't move receiver between threads cause trait Clone didn't implement for it thus we have to receive outside the spawning scope
// bot macros and functions
// ...
pub async fn ls(){}
pub async fn cd(){}