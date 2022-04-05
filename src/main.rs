use std::env::args;
use std::{process};
use dotenv::dotenv;
use colored::*;
use crossterm::style::Stylize;
use p2p_cli::Config;

#[macro_use]
extern crate dotenv_codegen;
fn main() {
    dotenv().ok();
    let args: Vec<String> = args().collect();
    let config = match Config::new(&args, dotenv!("SERVER_URL")) {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e.to_string().red());
            process::exit(1)
        }
    };
    println!("Welcome {}", config.username.bright_green());
    println!("Local peer id: {}", config.local_peer_id.to_string().bright_green());
    println!("Connecting to {} ...", config.server_endpoint.bright_green())
}
