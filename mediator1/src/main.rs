
mod trains;
mod train_station;

use train_station::TrainStation;
use trains::{FreightTrain, PassengerTrain};

fn main() {
    let train1 = FreightTrain::new("train1".into());
    let train2 = PassengerTrain::new("train2".into());

    let mut station = TrainStation::default();
    station.accept(train1);
    station.accept(train2);

    station.depart("train1");
    station.depart("train2");
}
