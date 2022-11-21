use crate::components::{CarType, Transmission, Engine};
use crate::car::Car;

pub trait Builder {
    type OutputType;
    fn set_car_type(&mut self, car_type : CarType);
    fn set_car_transmission(&mut self, car_transmission : Transmission);
    fn set_car_engine(&mut self, car_engine : Engine);

    fn build(self) -> Self::OutputType;
}

#[derive(Default)]
pub struct CarBuilder {
    car_type : Option<CarType>,
    transmission : Option<Transmission>,
    engine : Option<Engine>,
}

impl Builder for CarBuilder {
    type OutputType = Car;
    fn set_car_type(&mut self, car_type : CarType){
        self.car_type = Some(car_type);
    }

    fn set_car_transmission(&mut self, car_transmission : Transmission){
        self.transmission = Some(car_transmission);
    }

    fn set_car_engine(&mut self, car_engine : Engine){
        self.engine = Some(car_engine);
    }

    fn build(self) -> Car {
        Car::new(
            self.car_type.expect("set car type first"),
            self.transmission.expect("set transmission first"),
            self.engine.expect("set car engine first"),
        )
    }
}