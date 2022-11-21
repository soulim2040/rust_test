use crate::game::{Room, MazeGame};

#[derive(Clone)]
pub struct OrdinaryRoom {
    id : i32,
}

impl OrdinaryRoom {
    pub fn new(id : i32) -> Self {
        Self {id}
    }
}

impl Room for OrdinaryRoom {
    fn render(&self) {
        println!("ordinary room : {}", self.id);
    }
}

pub struct OrdinaryMaze {
    rooms : Vec<OrdinaryRoom>,
}

impl OrdinaryMaze {
    pub fn new() -> Self {
        Self {
            rooms : vec![
                OrdinaryRoom::new(11),
                OrdinaryRoom::new(22),
            ]
        }
    }
}

impl MazeGame for OrdinaryMaze {
    type RoomImpl = OrdinaryRoom;
    fn rooms(&self) -> Vec<Self::RoomImpl> {
        let mut rooms = self.rooms.clone();
        rooms.reverse();
        rooms
    }
}