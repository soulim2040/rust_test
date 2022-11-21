mod gui;
mod win_gui;
mod render;

use crate::gui::{GuiFactory};
use crate::win_gui::{WinGuiFactory};
use render::render;

fn main() {
    let fact = WinGuiFactory;
    render(fact);
}
