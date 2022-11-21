mod game;
mod magic_maze;
mod ordinary_maze;

use crate::magic_maze::MagicMaze;
use crate::ordinary_maze::OrdinaryMaze;
fn main() {
    let ordin = OrdinaryMaze::new();
    game::run(ordin);

    let magic = MagicMaze::new();
    game::run(magic);
}
