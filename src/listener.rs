use std::collections::LinkedList;
use irc::client::prelude::*;
use std::process::exit;
use std::io::{BufReader,BufWriter};
use irc::client::conn::NetStream;
use std::sync::mpsc::Sender;

pub struct Listener {
    outgoing: Sender<Message>,
    server: IrcServer<BufReader<NetStream>, BufWriter<NetStream>>,
}
           
impl Listener {
    // Iterate through messages pulled on IRC.
    // Put each one onto a queue that will be received by a parser
    pub fn start(&mut self) {
        for message in self.server.iter() {
            let message = message.unwrap(); // We'll just panic if there's an error.
            if message.command == "PRIVMSG" {
                // Do some sort of message processing
                self.outgoing.send(message.clone());
                info!(target: "receiver", "User {} said: {}", message.clone().get_source_nickname().unwrap(), message.suffix.unwrap());
            }
            else if message.command == "PING" {
                info!(target: "receiver", "Received PING, PONGing.");
                match self.server.send_pong("") {
                    Ok(_) => {},
                    Err(e) => {
                        println!("ERROR on PONG: {}", e);
                        exit(1);
                    }
                };
            }
        }
    }
    pub fn new(s: IrcServer<BufReader<NetStream>, BufWriter<NetStream>>, out: Sender<Message>) -> Listener {
        Listener { server: s, outgoing: out }
    }
}
