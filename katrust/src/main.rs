



// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
// https://doc.rust-lang.org/book/ch08-02-strings.html
// https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str/24159933#24159933
// https://learning-rust.github.io/docs/e4.unwrap_and_expect.html
// https://github.com/telegram-rs/telegram-bot
// https://github.com/teloxide/teloxide
// https://github.com/PyO3/pyo3
// https://github.com/wildonion/stomegranate
// https://github.com/v1s1t0r1sh3r3/airgeddon
// TODO - build something like laZagne and mimikatz
// TODO - bind this socket server in python using PyO3
// TODO - build a network monitoring handler like openwrt using tokio, a reverse proxy like ngrok and evil twins like airgeddon
// TODO - serve pre-trained models with rust using tch, actix, rocket, hyper, gotham, docker and k8s


// ==================================
// a simple socket server using tokio
// ==================================


use std::str;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	let addr = "0.0.0.0:8990".parse::<SocketAddr>().unwrap();
	let listener = TcpListener::bind(&addr).await.unwrap();
	println!("\n[+] Listening on: {}", addr);
    
	
    // ============= string testing
	    let name = String::from("erfan");
	    let another_name = "another erfan";
	    let combined = name + &another_name;
	    // name.push_str(&another_name); // name moved due to above operator
	    println!("{}", combined);
	    // println!("{}", name); // error - borrowed after move
	    println!("{}", another_name);

	    let sample_string = String::from("wildonion");
	    let bytes = sample_string.bytes(); // turn a string into buffer (asccii)
	    println!("[..] two first bytes of the string are : {}", &sample_string[0..2]); // byte indices
	    println!("[..] the string bytes : {:?}", bytes);
	
	let text = "hello hello from wildonion here double again again wildonion";
	let mut map = HashMap::new();
	for word in text.split_whitespace(){
		let count = map.entry(word).or_insert(0); // return a mutable reference inserted or the old value
		*count += 1; // updating the old value by dereferencing it, cause count is a mutable reference of the value 
	}

	println!("{:?}", map);
    // ==============
	
	
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


