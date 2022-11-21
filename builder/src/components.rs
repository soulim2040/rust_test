
#[derive(Debug)]
pub enum CarType {
    CityCar,
    SportCar,
    Suv,
}

#[derive(Debug)]
pub enum Transmission {
    SingleSpeed,
    Manual,
    Automatic,
    SemiAutomatic,
}

#[derive(Debug)]
pub struct Engine {
    volume      : f64,
    mileage     : f64,
    started     : bool,
}

impl Engine {
    pub fn new(vol : f64, mile : f64) -> Self {
        Self {
            volume : vol,
            mileage : mile,
            started : false,
        }
    }
}