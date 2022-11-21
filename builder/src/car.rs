use crate::components::{CarType, Transmission, Engine};
#[derive(Debug)]
pub struct Car {
    car_type : CarType,
    transmission : Transmission,
    engine : Engine,
}

impl Car {
    pub fn new(
        car_type : CarType,
        transmission : Transmission,
        engine : Engine,
    ) -> Self{
        Self {
            car_type,
            transmission, 
            engine,
        }
    }
}