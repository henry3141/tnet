use crate::datastructure::*;
use serde_json;
#[get("/command/<set>/<id>")]
fn command(set:i32,id:i32) -> String {
    let command = CommandLevelTwo::Goto((0,0,0));
    serde_json::to_string(&command).unwrap()
}

#[post("/update/<set>/<id>", data = "<data>")]
fn update(set:i32,id:i32,data:String) -> String {
    let turtle:Turtle = serde_json::from_str(&data).unwrap();
    "true".to_string()
}

#[post("/world", data = "<data>")]
fn world(data:String) -> String {
    let block:Vec<BlockNet> = serde_json::from_str(&data).unwrap();
    "true".to_string()
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![command])
        .mount("/", routes![update])
        .mount("/", routes![world])
}