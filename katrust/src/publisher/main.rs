





/*




    we can’t just pass the receiver between multiple threads cause trait Copy is not implemented for the receiver thus we can’t clone it to fix the issue cause if a type is Copy its Clone needs to return *self.
    Multiple producer means multiple threads own the receiver and single consumer means only one of them can mutate and get the job or task from the receiver at a time.
    to fix the issue we have to take an atomic reference from the receiver using Arc in order to clone it for passing between multiple threads and for mutating it we have to 
    put it inside a Mutex to insure that only one thread can change the content of the receiver at a time. this is done by waiting on the receiver until a job or task becomes 
    available to the down side of the channel then locking on the receiver to acquire the mutex.
    the receiver of tokio mpsc channel is shareable between tokio::spawn() threads so we don’t need to take an atomic reference and put it inside the Mutex.





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






TODO - custom derive macro or proc_macro for my own traits using trait scope orphan rule, closures, asynchronous (async move) and multithreaded pattern
TODO - smart pointer like Rc, Arc, RwLck, RefCell and Mutex
TODO - anomaly detection in chromosomes
TODO - multithreading server for backdoor





*/




mod handlers;
use std::env;
use dotenv::dotenv;





#[tokio::main] //-- await is only allowd inside an async function due to this reason we're using the tokio as a runtime to make the main() function as an async one
async fn main() -> std::io::Result<()>{
    
    

    
    env::set_var("RUST_LOG", "librdkafka=trace,rdkafka::client=debug");
    env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let host = env::var("KAFKA_HOST").expect("⚠️ please set host in .env");
    let secret_key = env::var("SECRET_KEY").expect("⚠️ found no jwt secret key"); // TODO - 
    let node1_port = env::var("KAFKA_NODE1_PORT").expect("⚠️ please set kafka node1 port in .env"); //-- broker 1
    let node2_port = env::var("KAFKA_NODE2_PORT").expect("⚠️ please set kafka node2 port in .env"); //-- broker 2
    let node3_port = env::var("KAFKA_NODE3_PORT").expect("⚠️ please set kafka node3 port in .env"); //-- broker 3




            {
                'outer: loop{
                    println!("this is the outer loop");
                    'inner: loop{
                        println!("this is the inner loop");
                        // break; // only the inner loop
            
                        break 'outer;
                    }
            
                    println!("this print will never be reached");
                }
            
            
            
            
                let mut counter = 0;
                let result = loop{
                    counter += 1;
                    if counter == 10{
                        break counter * 2;
                    }
                };
                println!("counter is {}", counter);
                println!("result is {}", result);

            }





    let broker1 = format!("{}:{}", host, node1_port);
    let broker2 = format!("{}:{}", host, node2_port);
    let broker3 = format!("{}:{}", host, node3_port);
    let brokers = format!("{},{},{}", broker1, broker2, broker3);
    handlers::producer::produce(&brokers).await; //-- passing brokers String by taking a reference to it, by doing this we're coercing it into &str - &String is &str


    


    Ok(())



}