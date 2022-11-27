use crate::trains::Train;
use crate::train_station::Mediator;

pub struct PassengerTrain {
    name: String,
}

impl PassengerTrain {
    pub fn new(name: String) -> Self {
        Self {name}
    }
}

impl Train for PassengerTrain {
    fn name(&self)  -> &String{
        &self.name
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator){
        if !mediator.notify_about_arrival(&self.name) {
            println!("passenger train {} : arrival block, waiting...", self.name);
            return;
        }
        println!("passenger train {} : arrival ", self.name);
    }
    
    fn depart(&mut self, mediator: &mut dyn Mediator){
        mediator.notify_about_departure(&self.name);
        println!("passenger train {} : departure ", self.name);
    }
}