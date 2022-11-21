use crate::gui::{Button, CheckBox, GuiFactory};

pub struct WinButton;

impl Button for WinButton {
    fn press(&self) {
        println!("press win button");
    }
}

pub struct WinCheckBox;

impl CheckBox for WinCheckBox {
    fn switch(&self){
        println!("switch win check box");
    }
}

pub struct WinGuiFactory;

impl GuiFactory for WinGuiFactory {
    type B = WinButton;
    type C = WinCheckBox;

    fn create_button(&self) -> WinButton {
        WinButton
    }

    fn create_checkbox(&self) -> WinCheckBox {
        WinCheckBox
    }
}