use std::{fmt, io};
use std::error::Error;
use std::io::ErrorKind;
use colored::Colorize;

use libp2p::{identity, PeerId};
use libp2p::identity::Keypair;

#[derive(Debug, Clone)]
struct InvalidUsernameError {
    username: String
}

impl Error for InvalidUsernameError {}

impl fmt::Display for InvalidUsernameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Invalid username - has to be at least 3 characters long, got {}", self.username.len()))
    }
}

pub struct Config {
    pub server_endpoint: String,
    pub username: String,
    pub local_peer_id: PeerId,
    pub local_key: Keypair
}

impl Config {
    pub fn new(args: &[String], server_endpoint: &str) -> Result<Config, Box<dyn std::error::Error>>{

        let username = match get_username(args.get(1)){
            Ok(v) => v,
            Err(e) => return Err(Box::new(e))
        };

        if username.len() < 3 {
            return Err(Box::new(InvalidUsernameError{username}));
        }

        // Create a random PeerId
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        Ok(Config{server_endpoint: server_endpoint.to_string(), username, local_peer_id, local_key })
    }
}

pub fn get_username(username_option: Option<&String>) -> Result<String, io::Error> {
    match username_option
    {
        Some(v) => Ok(v.clone()),
        None => get_input("Please type your username")
    }
}

pub fn get_input(prompt: &str) -> Result<String, io::Error >{
    println!("{}",prompt.bright_green());
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();
            if input.len() <= 0 {
                return Err(io::Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid username",
                ))
            }
            Ok(input)
        },
        Err(error) => Err(error),
    }
}