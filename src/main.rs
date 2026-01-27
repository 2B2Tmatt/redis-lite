use std::io::{self, Write};

use crate::{protocol::parse_line, store::Store};
mod protocol;
mod store;
fn main() {
    println!("redis-lite started >>");
    let mut store = Store::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let res = parse_line(&input);
        let cmd = match res {
            Ok(cmd) => cmd,
            Err(msg) => {
                println!("command parsing failed with message: {msg}");
                continue;
            }
        };

        let res = store.apply(cmd);
        println!("{res}")
    }
}
