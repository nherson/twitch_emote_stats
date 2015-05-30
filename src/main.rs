extern crate irc;

use std::process::exit;
use std::path::Path;
use irc::client::prelude::*;

fn main() {
    let config = match Config::load(Path::new("etc/conf.json")) {
        Ok(c) => c,
        Err(e) => {
            println!("ERROR: {}", e); 
            exit(1);
        }
    };
    // Clone the nickname and password so we can manually send our auth
    // in the specific way that Twitch expects it. 
    // (else they are moved into the from_config method).
    let nickname = config.nickname.clone().unwrap().to_string();
    let password = config.password.clone().unwrap().to_string();
    let server = IrcServer::from_config(config).unwrap();
    match server.send(Command::PASS(password)) {
        Ok(_) => {},
        Err(e) => {
            println!("ERROR: {}", e);
            exit(1);
        }
    };
    match server.send(Command::NICK(nickname)) {
        Ok(_) => {},
        Err(e) => {
            println!("ERROR: {}", e);
        }
    };
    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        if message.command == "PRIVMSG" {
            // Do some sort of message processing
            print!("User {} said: ", message.get_source_nickname().unwrap());
            print!("{}", message.suffix.unwrap());
        }
        else if message.command == "PING" {
            println!("Received PING, PONGing.");
            match server.send_pong("") {
                Ok(_) => {},
                Err(e) => {
                    println!("ERROR on PONG: {}", e);
                    exit(1);
                }
            };
        }
    }
}
