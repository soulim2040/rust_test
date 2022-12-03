mod remotes;
mod devices;

use devices::{Device, Radio, Tv};
use remotes::{Remote, BasicDevice, AdvanedDevice, HasMutDevice};


fn main() {
    test_device(Tv::default());
    test_device(Radio::default());
}

fn test_device(device: impl Device + Clone){
    println!("Tests with basic remote.");
    let mut basic_remote = BasicDevice::new(device.clone());
    basic_remote.power();
    basic_remote.device().print_status();

    println!("Tests with advanced remote.");
    let mut advanced_remote = AdvanedDevice::new(device);
    advanced_remote.power();
    advanced_remote.mute();
    advanced_remote.device().print_status();
}