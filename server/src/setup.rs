use std::collections::HashMap;
use crate::datastructure::*;
use crate::server;
use std::thread;


impl Tnet {
    pub fn new() -> Tnet {
        Tnet {
            sets: HashMap::new(),
            world: HashMap::new(),
            commands: HashMap::new(),
        }
    }


    ///blocks the current thread 
    pub async fn run(&mut self) {
        thread::spawn(move ||  async {});
        server::rocket().launch().await.unwrap();
    }
}