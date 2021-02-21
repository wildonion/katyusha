


// https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str/24159933#24159933
// https://learning-rust.github.io/docs/e4.unwrap_and_expect.html
// https://github.com/telegram-rs/telegram-bot
// https://github.com/teloxide/teloxide
// https://github.com/PyO3/pyo3
// TODO - bind this socket server in python using PyO3


// ==================================
// a simple socket server using tokio
// ==================================

use std::str;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	let addr = "0.0.0.0:8990".parse::<SocketAddr>().unwrap();
	let listener = TcpListener::bind(&addr).await.unwrap();
	println!("\n[+] Listening on: {}", addr);
    let sample_string = String::from("wildonion");
    let bytes = sample_string.as_bytes(); // turn a string into buffer (asccii)
    format!("two first bytes of sample_string are : {}", &sample_string[0..2]); // indices here are byte indices

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("[+] Socket info: {:#?}", socket);

        tokio::spawn(async move { // move the socket to the async and spawned scope
            let mut buf = vec![0; 6400]; // allocating the buffer
            loop {
                let n = match socket.read(&mut buf).await {              
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("[!] Failed to read from socket; err = {:?}", e);
                        return;
                    }
                };		
		    		// NOTE - we can use match or unwrap()
				// let data = str::from_utf8(&buf[0..n]).unwrap();
				let data = match str::from_utf8(&buf[0..n]) {
				        Ok(v) => v,
				        Err(e) => panic!("[!] Invalid UTF-8 sequence: {}", e),
				    };
				println!("[+] Request: {}", data);

				    
                if let Err(e) = socket.write_all(b"\n[+] Response: OK\n").await {
                    eprintln!("[!] Failed to write to socket; err = {:?}", e);
                    return;
                }
            } // loop end
        }); // tokio spawn end
    } // loop end
}


