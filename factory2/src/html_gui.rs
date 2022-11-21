
use crate::gui::{Button, Dialog};

pub struct HtmlButton;

impl Button for HtmlButton {
    fn render(&self) {
        println!("<button> html button</button>");
    }

    fn on_click(&self) {
        println!("html button on click");
    }
}

pub struct HtmlDialog;

impl Dialog for HtmlDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(HtmlButton)
    }
}