use std::collections::LinkedList;
use std::sync::{Arc,Mutex};
use irc::client::prelude::*;
use std::sync::mpsc::{Receiver,Sender};

pub struct Parser {
    incoming: Receiver<Message>,
    outgoing: Sender<String>,
    emotes: Vec<String>,
}

impl Parser {
    pub fn new(inc: Receiver<Message>, out: Sender<String>) -> Parser {
        // Do some business to load the emotes
        Parser { incoming: inc, outgoing: out, emotes: vec!["Kappa".to_string(), "EleGiggle".to_string(), "BibleThump".to_string()] }
    }

    // - For each message
    //   - Split it into words (split on whitespace)
    //   - check each word against some list of desired emotes
    //   - if word is an emote:
    //     - get timestamp and push emote/timestamp pair onto logging queue
    pub fn start(&mut self) {
        loop {
            let message = self.incoming.recv().unwrap();
            let line = message.suffix.clone().unwrap();
            for word in line.split(" ") {
                for emote in self.emotes.clone() {
                    if emote == word {
                        println!("FOUND: {} by {}", word, message.get_source_nickname().unwrap());
                    }
                }
            }
        }
    }

}
