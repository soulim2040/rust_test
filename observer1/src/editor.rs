use crate::observer::{Event, Publisher};

#[derive(Default)]
pub struct Editor{
    publisher : Publisher,
    file_path : String,
}

impl Editor {
    pub fn publish_er(&mut self) -> &mut Publisher{
        &mut self.publisher
    }

    pub fn load(&mut self, path : String){
        self.file_path = path.clone();
        self.publisher.notify(Event::Load, path.clone());
    }

    pub fn save(&self){
        self.publisher.notify(Event::Save, self.file_path.clone());
    }
}