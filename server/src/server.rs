use std::sync::{Mutex, Arc};

use crate::datastructure::*;
use rocket::State;
use serde_json;

#[get("/command/<set>/<id>")]
pub fn command(set:i32,id:i32,tnet: &State<Arc<Mutex<Tnet>>>) -> String {
    let command = CommandLevelTwo::Goto((20,-20,0));
    serde_json::to_string(&command).unwrap()
}

#[post("/update/<set>/<id>", data = "<data>")]
pub fn update(set:i32,id:i32,data:String,tnet: &State<Arc<Mutex<Tnet>>>) -> String {
    let turtle:Turtle = serde_json::from_str(&data).unwrap();
    "true".to_string()
}

#[post("/world", data = "<data>")]
pub fn world(data:String,tnet: &State<Arc<Mutex<Tnet>>>) -> String {
    let blocks:Vec<BlockNet> = serde_json::from_str(&data).unwrap();
    "true".to_string()
}

#[post("/register",data="<data>")]
pub fn register(data:String,tnet: &State<Arc<Mutex<Tnet>>>) -> String {
    let turtle:Turtle = serde_json::from_str(&data).unwrap();
    "{\"id\":0,\"set\":0}".to_string()
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![command])
        .mount("/", routes![update])
        .mount("/", routes![world])
}