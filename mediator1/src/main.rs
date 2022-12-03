
mod trains;
mod train_station;

use train_station::TrainStation;
use trains::{FreightTrain, PassengerTrain, GenericTrain, TrainType};

fn main() {
    let train1 = FreightTrain::new("train1".into());
    let train2 = PassengerTrain::new("train2".into());

    let mut station = TrainStation::default();
    station.accept(train1);
    station.accept(train2);

    station.depart("train1");
    station.depart("train2");

    println!("================");
    let train3 = GenericTrain::new("train3".into(), TrainType::FreightType);
    let train4 = GenericTrain::new("train4".into(), TrainType::PassengerType);


    station.accept(train3);
    station.accept(train4);

    station.depart("train3");
    station.depart("train4");
}
