mod copy;
mod cut;
mod paste;

pub trait Command {
    fn execute(&mut self, app : &mut cursive::Cursive) -> bool;
    fn undo(&mut self, app : &mut cursive::Cursive);
}

pub use copy::CopyCommand;
pub use cut::CutCommand;
pub use paste::PasteCommand;

