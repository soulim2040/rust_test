
pub trait Room {
    fn render(&self);
}

pub trait MazeGame {
    // 这里的语法有点不解, 不能用=号
    type RoomImpl : Room;
    fn rooms(&self) -> Vec<Self::RoomImpl>;

    fn play(&self) {
        for room in self.rooms() {
            room.render();
        }
    }
}

pub fn run(maze_game : impl MazeGame){
    maze_game.play();
}