#[macro_use]
extern crate log;
extern crate irc;
extern crate env_logger;

use std::process::exit;
use std::path::Path;
use irc::client::prelude::*;
use std::collections::LinkedList;
use std::error::Error;
use log::LogLevel;
use std::sync::mpsc::{Sender,Receiver,channel};
use std::thread;

mod listener;
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
    let (listener_out, parser_in) = channel();
    let (parser_out, logger_in) = channel();
    let mut lst = listener::Listener::new(server, listener_out);
    let mut prsr = parser::Parser::new(parser_in, parser_out);
    // start the parser
    let p = thread::spawn(move || {
        prsr.start();
    });
    // start the listener
    let l = thread::spawn(move || {
        lst.start();
    });
    p.join();
    l.join();
}

fn handle_error(e: &Error) -> ! {
    println!("ERROR: {}", e.description());
    exit(1);
}
