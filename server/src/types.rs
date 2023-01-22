use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::promise::Promise;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Block {
    Air,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turtle {
    pub id:i32,
    pub position:(i32,i32,i32),
    pub inventory:HashMap<String,i32>,
    pub fuel:i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turtles {
    pub inactives:Vec<Turtle>,
    pub actives:Vec<Turtle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldUpdate {
    BlockUpdate((i32,i32,i32),Block),
    TurtleUpdate(Turtle),
}