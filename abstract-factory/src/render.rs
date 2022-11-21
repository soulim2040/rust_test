use crate::gui::{GuiFactory, Button, CheckBox};

pub fn render(factory : impl GuiFactory){
    let btn1 = factory.create_button();
    let chk1 = factory.create_checkbox();

    btn1.press();
    chk1.switch();
}