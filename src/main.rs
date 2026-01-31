use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::{protocol::parse_line, store::Response, store::Store};
mod protocol;
mod store;
fn main() {
    let store = Store::new();
    let store = Arc::new(Mutex::new(store));
    const ADDR: &str = "127.0.0.1:8080";
    start_server(ADDR, store);
}
fn start_server(addr: &str, store: Arc<Mutex<Store>>) {
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");
    println!("Server listening on {addr}");
    println!("redis-lite started >>");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let thread_id = get_unique_id();
                println!("new connection opened, thread id: {thread_id}");
                let store = Arc::clone(&store);
                thread::spawn(move || match handle_connection(stream, store) {
                    Err(e) => {
                        eprintln!("connection error: {e}");
                    }
                    Ok(_) => {
                        println!("connection with thread id: {thread_id} closed");
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {e}")
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, store: Arc<Mutex<Store>>) -> io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    'conn: loop {
        write!(&stream, "> ")?;
        stream.flush().expect("failed to flush stdout");
        let mut input = String::new();
        reader.read_line(&mut input)?;
        let res = parse_line(&input);
        let cmd = match res {
            Ok(cmd) => cmd,
            Err(msg) => {
                writeln!(&stream, "command parsing failed with message: {msg}")?;
                continue;
            }
        };

        let mut store = store.lock().unwrap();
        let res = store.apply(cmd);
        match res {
            Response::Simple(s) => {
                writeln!(&stream, "{s}")?;
            }
            Response::Integer(i) => {
                writeln!(&stream, "{i}")?;
            }
            Response::Error(e) => {
                writeln!(&stream, "error: {e}")?;
            }
            Response::Bulk(b) => {
                writeln!(&stream, "{b}")?;
            }
            Response::List(l) => {
                for (i, e) in l.iter().enumerate() {
                    writeln!(&stream, "{i}) {e}")?;
                }
            }
            Response::Quit() => {
                writeln!(&stream, "OK")?;
                break 'conn;
            }
        }
    }
    Ok(())
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn get_unique_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed) + 1
}
