mod db;
mod handler;
mod utils;

use clap::Parser;
use dotenv::dotenv;
use human_panic::setup_panic;

// cli params
#[derive(Debug, Parser)]
#[command(name = "MiniKey", author = "", version, about = "a simple kv store")]
struct Args {
    #[arg(required = false)]
    cmd: Option<String>,

    #[arg(short, long)]
    custom: Option<String>,

    #[arg(short, long)]
    docs: bool,
}

#[tokio::main]
async fn main() {
    // load env
    dotenv().ok();

    // setup panic for read
    setup_panic!();

    let logo = r#"
    ___  ____       _   _              
    |  \/  (_)     (_) | |             
    | .  . |_ _ __  _  | | _____ _   _ 
    | |\/| | | '_ \| | | |/ / _ \ | | |
    | |  | | | | | | | |   <  __/ |_| |
    \_|  |_/_|_| |_|_| |_|\_\___|\__, |
                                __/ |
                                |___/ 
    "#;

    bunt::println!("{$green}{}{/$}", logo);
    let args = Args::parse();
    let cmd: String;
    if args.cmd.is_some() {
        cmd = args.cmd.unwrap();
    } else {
        cmd = inquire::Text::new("type command: ")
            .with_help_message("please type a useful command")
            .with_autocomplete(&utils::suggester)
            .prompt()
            .unwrap();
    }

    println!("this command is: {}", cmd);

    match cmd.as_str() {
        "set" => handler::add().await,
        "list" => handler::list().await,
        "delete" => handler::delete().await,
        "get" => handler::get().await,
        "search" => handler::search().await,
        "exit" => {
            bunt::println!("{$red}Exiting...{/$}");
            std::process::exit(0);
        }
        "help" => todo!("help command in progress"),
        _ => todo!("command not found"),
    }
}
