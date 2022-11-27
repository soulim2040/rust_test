use crate::trains::Train;
use crate::train_station::Mediator;

pub struct FreightTrain {
    name: String,
}

impl FreightTrain {
    pub fn new(name: String) -> Self {
        Self {name}
    }
}

impl Train for FreightTrain {
    fn name(&self)  -> &String{
        &self.name
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator){
        if !mediator.notify_about_arrival(&self.name) {
            println!("freight train {} : arrival block, waiting...", self.name);
            return;
        }
        println!("freight train {} : arrival ", self.name);
    }
    
    fn depart(&mut self, mediator: &mut dyn Mediator){
        mediator.notify_about_departure(&self.name);
        println!("freight train {} : departure ", self.name);
    }
}