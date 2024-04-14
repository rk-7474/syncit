use std::env;
use syncit::auth::read_token;
use syncit::sync;
use syncit::auth;
use async_std;

const AUTH_COMMANDS: &'static [&'static str] = &[
    "get", 
    "send"
];

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
    let mut path: String;
    let drawer: String;

    if arg.is_some() {
        path = arg.unwrap();
        
        if path.chars().last().unwrap() != '/' {
            path = format!("{path}/");
        }

        drawer = path.rsplit_once('/').unwrap().0.to_string().rsplit_once('/').unwrap().1.to_string();
    } else {
        let drawer_path = std::env::current_dir().unwrap().display().to_string();
        drawer = drawer_path.clone().split_off(drawer_path.rfind("/").unwrap() + 1); 

        path = "./".to_string();
    };

    if read_token().is_empty() && AUTH_COMMANDS.iter().any( |&i| i == command ) {
        println!("You must be authenticated to run this command. Run `syncit token` to insert your auth token.");
        return;
    }

    match command {
        "send" => handle_result(sync::send(&drawer, path).await, "Error sending data"),
        "get" => handle_result(sync::get(&drawer, path).await, "Error getting data"),
        "token" => handle_result(auth::set_token().await, "Error while setting token"),
        _ => println!("Invalid command"),
    }
    
    fn handle_result<T>(result: Result<T, impl std::fmt::Display>, error_message: &str) {
        if let Err(e) = result {
            eprintln!("{}: {}", error_message, e);
        }
    }
}