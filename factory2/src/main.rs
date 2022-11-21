mod gui;
mod html_gui;
mod windows_gui;

use crate::gui::Dialog;
use crate::html_gui::HtmlDialog;

fn main() {
    let html_dlg = HtmlDialog{};
    html_dlg.render();
}
