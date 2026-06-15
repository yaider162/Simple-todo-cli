use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

impl Task{
    pub fn new(id:i32, title:String, done: bool) -> Self {
        Self {id, title, done}
    }
}