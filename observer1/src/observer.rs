use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Event {
    Load,
    Save,
}

pub type Subscriber = fn(file_path : String);

#[derive(Default)]
pub struct Publisher {
    events : HashMap<Event, Vec<Subscriber>>,
}

impl Publisher {
    pub fn subscribe(&mut self, event : Event, listener : Subscriber){
        self.events.entry(event).or_default();
        self.events.get_mut(&event).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event : Event, listener : Subscriber){
        self.events.entry(event).or_default();
        self.events.get_mut(&event).unwrap().retain(|&x| x != listener);
    }

    pub fn notify(&self, event : Event, file_path : String){
        let listeners = self.events.get(&event).unwrap();
        for listener in listeners{
            listener(file_path.clone());
        }
    }
}