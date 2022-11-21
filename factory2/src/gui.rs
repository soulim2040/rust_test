pub trait Button {
    fn render(&self);
    fn on_click(&self);
}

pub trait Dialog {
    fn create_button(&self) -> Box<dyn Button>;

    fn render(&self) {
        let btn = self.create_button();
        btn.render();
    }

    fn refresh(&self) {
        println!("dialog refresh");
    }
}