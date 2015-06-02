use std::collections::LinkedList;
use std::sync::{Arc,Mutex};
use irc::client::prelude::*;

struct Parser {
    incoming: Receiver<Message>,
    outgoing: Sender<String>,
    emotes: Vec<String>,
}

impl Parser {
    pub fn new(pq: Arc<Mutex<LinkedList<Message>>>) -> Parser {
        // Do some business to load the emotes
        Parser { parse_queue: pq, emotes: vec!["Kappa", "EleGiggle", "BibleThump"] }
    }

    // - For each message
    //   - Split it into words (split on whitespace)
    //   - check each word against some list of desired emotes
    //   - if word is an emote:
    //     - get timestamp and push emote/timestamp pair onto logging queue
    pub fn start(&mut self) {
        loop {
            let message = incoming.recv();
            let re = regex!(r" +");
            let words = re.replace_all(message.suffix.clone().unwrap(), " ").unwrap().split(" ");
            for word in words {
                for emote in self.emotes {
                    if word == emote {
                        println!("FOUND: {}", word);
                    }
                }
            }
        }
    }

}
