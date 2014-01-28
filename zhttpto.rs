//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut hitcount: int = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            unsafe { hitcount += 1; }
            let split_request: ~[&str] = request_str.words().collect();
            if (split_request.len() > 3 &&
                    split_request[0] == "GET" &&
                    split_request[1].len() > 1 &&
                    split_request[2] == "HTTP/1.1") {
                if split_request[1].ends_with(".html") {
                    let relpath = Path::new(split_request[1].slice_from(1));
                    match result(|| File::open(&relpath)) {
                        Ok(mut f) => {
                            let response: ~[u8] = f.read_to_end();
                            stream.write(response);
                        } ,
                        Err(e) => {
                            if e.kind == PermissionDenied {
                                stream.write("403".as_bytes());
                            } else if e.kind == FileNotFound {
                                stream.write("404".as_bytes());
                            } else {
                                stream.write("io error".as_bytes());
                            }
                        }
                    }
                } else {
                    stream.write("403".as_bytes());
                }
            } else {
                unsafe {
                let response: ~str = 
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                     <doctype !html><html><head><title>Hello, Rust!</title>
                     <style>body \\{ background-color: \\#111; color: \\#FFEEAA \\}
                            h1 \\{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red\\}
                            h2 \\{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green\\}
                     </style></head>
                     <body>
                     <h1>Greetings, Krusty!</h1>
                     <h2>Count: {:d}</h2>
                     </body></html>\r\n", hitcount);
                stream.write(response.as_bytes());
                }
            }
            println!("Connection terminates.");
        }
    }
}
