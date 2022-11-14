use game::{Room, MazeGame};
use magic_maze::MagicMaze;
use ordinary_maze::OrdinaryMaze;
fn main() {
    let ordin = OrdinaryMaze::new();
    game:run(ordin);

}
