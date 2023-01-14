use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone , Serialize, Deserialize , PartialEq)]
pub struct Tnet {
    pub sets: HashMap<i32,Set>,
    pub world:HashMap<(i32,i32,i32),Block>,
    pub commands:HashMap<i32,CommandLevelOne>,
}

#[derive(Debug, Clone , Serialize, Deserialize , PartialEq)]
pub struct Set {
    pub turtles:[Turtle;10],
    pub commands:CommandLevelOne,
}

#[derive(Debug, Clone , Serialize, Deserialize , PartialEq)]
pub struct Turtle {
    pub position:(i32,i32,i32),
    pub commands:CommandLevelTwo,
    pub fuel:i32,
    pub inventory:[Item;10],
}

#[derive(Debug, Clone , Serialize, Deserialize , PartialEq)]
pub enum Block {
    Air,
    Stone,
    Dirt,
}

#[derive(Debug, Clone , Serialize, Deserialize , PartialEq)]
pub struct BlockItem {
    pub block:Block,
    pub amount:i32,
}

#[derive(Debug, Clone , Serialize, Deserialize , PartialEq)]
pub enum Item {
    Block(BlockItem),
}

#[derive(Debug, Clone , Serialize, Deserialize , PartialEq)]
pub enum CommandLevelOne {
    Get(Item,(i32,i32,i32)),
    Remove((i32,i32,i32),(i32,i32,i32)),
    Move((i32,i32,i32),i32),
}

#[derive(Debug, Clone , Serialize, Deserialize , PartialEq)]
pub enum CommandLevelTwo {
    Goto((i32,i32,i32)),
    MineGoto(Vec<(i32,i32,i32)>),
    Craft(Item),
    Share(Item),
    Receive(Item),
    Cancel,
}

#[derive(Debug, Clone , Serialize, Deserialize , PartialEq)]
pub struct BlockNet {
    pub position:(i32,i32,i32),
    pub block:Block,
}