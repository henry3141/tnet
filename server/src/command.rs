use std::collections::HashMap;
use crate::types::*;

pub trait Command {
    fn select_turtles(&self,turtles:Vec<Turtle>) -> Vec<Turtle>;
    fn create(data:HashMap<String,String>) -> Self;
    fn set_turtles(&mut self,turtles:Vec<Turtle>);
    fn get_turtles(&self) -> Vec<Turtle>;
    fn get_command(&self,id:i32) -> String;
    fn update(&mut self,u:WorldUpdate);
}
