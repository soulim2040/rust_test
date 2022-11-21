use crate::game::{Room, MazeGame};

#[derive(Clone)]
pub struct MagicRoom {
    title : String,
}

impl MagicRoom {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

impl Room for MagicRoom {
    fn render(&self){
        println!("magic room : {}", self.title);
    }
}

pub struct MagicMaze {
    rooms : Vec<MagicRoom>,
}

impl MagicMaze {
    pub fn new() -> Self {
        Self {
            rooms : vec![
                MagicRoom::new("magic 11".into()),
                MagicRoom::new("magic 22".into()),
            ],
        }
    }
}

impl MazeGame for MagicMaze {
    type RoomImpl = MagicRoom;

    fn rooms(&self) -> Vec<Self::RoomImpl> {
        self.rooms.clone()
    }
}