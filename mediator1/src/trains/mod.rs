mod freight_train;
mod passenger_train;

pub use freight_train::FreightTrain;
pub use passenger_train::PassengerTrain;

use crate::train_station::Mediator;

pub trait Train {
    fn name(&self)  -> &String;
    fn arrive(&mut self, mediator: &mut dyn Mediator);
    fn depart(&mut self, mediator: &mut dyn Mediator);
}

pub enum TrainType {
    FreightType, 
    PassengerType,
}

pub struct GenericTrain{
    name : String,
    train_type : TrainType,
}

impl GenericTrain{
    pub fn new(name: String, train_type : TrainType) -> Self{
        Self { name, train_type}
    }

    fn get_train_type_name(&self) -> &'static str {
        match self.train_type {
            TrainType::FreightType => "freight",
            TrainType::PassengerType => "passenger",
            _ => unimplemented!(),
        }
    }
}

impl Train for GenericTrain {
    fn name(&self)  -> &String{
        &self.name
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator){
        let gen_name = self.get_train_type_name();
        if !mediator.notify_about_arrival(&self.name) {
            println!("{} train {} : arrival block, waiting...", gen_name, self.name);
            return;
        }
        println!("{} train {} : arrival ", gen_name, self.name);
    }
    
    fn depart(&mut self, mediator: &mut dyn Mediator){
        let gen_name = self.get_train_type_name();
        mediator.notify_about_departure(&self.name);
        println!("{} train {} : departure ", gen_name, self.name);
    }
}