use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use crate::datastructure::*;
use crate::server::*;
use std::thread;


impl Tnet {
    ///blocks the current thread 
    pub async fn run() {
        let tnet = Tnet {
            sets: HashMap::new(),
            world: HashMap::new(),
            commands: HashMap::new(),
        };
        let tnet = Arc::new(Mutex::new(tnet));
        let net2 = tnet.clone();
        thread::spawn(move ||  async {
            let mut tnet = net2;
        });
        let _ =  rocket::build()
            .mount("/", routes![command])
            .mount("/", routes![update])
            .mount("/", routes![world])
            .mount("/", routes![register])
            .manage(tnet.clone())
            .launch().await.unwrap();
    }
}