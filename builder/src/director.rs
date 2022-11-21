
use crate::builder::car_builder::{Builder, CarBuilder};
use crate::components::{CarType, Transmission, Engine};
pub struct Director;

impl Director {
    pub fn construct_sprots_car(builder : &mut impl Builder) {
        builder.set_car_type(CarType::SportCar);
        builder.set_car_transmission(Transmission::SemiAutomatic);
        builder.set_car_engine(Engine::new(1.0, 0.0));
    }
}