#[macro_use]
extern crate regex;
extern crate log;
extern crate irc;
extern crate env_logger;


use std::process::exit;
use std::path::Path;
use irc::client::prelude::*;
use std::collections::LinkedList;
use std::error::Error;
use log::LogLevel;

mod receiver;
mod parser;

fn main() {
    // Setup a logger
    env_logger::init().unwrap();

    let config = match Config::load(Path::new("etc/conf.json")) {
        Ok(c) => c,
        Err(e) => handle_error(&e),
    };
    // Clone the nickname and password so we can manually send our auth
    // in the specific way that Twitch expects it. 
    // (else they are moved into the from_config method).
    let nickname = config.nickname.clone().unwrap().to_string();
    let password = config.password.clone().unwrap().to_string();
    let server = IrcServer::from_config(config).unwrap();
    match server.send(Command::PASS(password)) {
        Ok(_) => {},
        Err(e) => handle_error(&e),
    };
    match server.send(Command::NICK(nickname)) {
        Ok(_) => {},
        Err(e) => handle_error(&e),
    };
    let parse_queue = LinkedList::new();
    let mut recv = receiver::Receiver::new(server, parse_queue);
    recv.start();
}

fn handle_error(e: &Error) -> ! {
    println!("ERROR: {}", e.description());
    exit(1);
}
