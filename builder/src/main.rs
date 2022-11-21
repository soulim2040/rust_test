
mod car;
mod components;
mod builder;
mod director;

use crate::builder::car_builder::Builder;
use crate::builder::car_builder::CarBuilder;
use crate::director::Director;

fn main(){
    let mut builder = CarBuilder::default();
    Director::construct_sprots_car(&mut builder);
    let car = builder.build();
    println!("{:?}", car);
}    