mod handler;

use std::eprint;
use std::net::TcpListener;
use threadpool::ThreadPool;
use clap::Parser;
use crate::handler::handle_connection;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short='H', long, default_value_t=String::from("127.0.0.1"))]
    host: String,

    #[arg(short, long, default_value_t=5000)]
    port: u16
}

fn main() {
    let args = Args::parse();
    let listener = TcpListener::bind(format!("{}:{}", args.host, args.port)).unwrap();
    let pool = ThreadPool::new(4);

    println!("Listening on http://{}:{}/", args.host, args.port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprint!("Error in stream: {}", e);
            }
        }
    }
}