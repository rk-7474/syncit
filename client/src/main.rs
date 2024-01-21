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

    let first = args.nth(1).unwrap();
    let command = first.as_str();

    let arg: Option<String> = args.nth(0);
    let path: String;
    let drawer: String;

    if arg.is_some() {
        path = arg.unwrap();
        drawer = path.rsplit_once('/').unwrap().0.to_string().rsplit_once('/').unwrap().1.to_string();
    } else {
        let drawer_path = std::env::current_dir().unwrap().display().to_string();
        drawer = drawer_path.clone().split_off(drawer_path.rfind("/").unwrap() + 1); 

        path = "./".to_string();
    };
    
    match command {
        "send" => handle_result(sync::send(&drawer, path).await, "Error sending data"),
        "get" => handle_result(sync::get(&drawer, path).await, "Error getting data"),
        _ => println!("Invalid command"),
    }
    
    fn handle_result<T>(result: Result<T, impl std::fmt::Display>, error_message: &str) {
        if let Err(e) = result {
            eprintln!("{}: {}", error_message, e);
        }
    }
}