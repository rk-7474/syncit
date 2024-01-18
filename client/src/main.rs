use std::env;
use syncit::sync;
use async_std;

#[async_std::main]
async fn main() {
    let mut args = env::args();
    let len = args.len();
    
    if len < 2 {
        println!("No command included.");
        return;
    }

    match args.nth(1).unwrap().as_str() {
        "sync" => sync::add(&mut args).await,
        _ => println!("Invalid command")
        // "drop" => sync::remove(&mut args),
    }
}
