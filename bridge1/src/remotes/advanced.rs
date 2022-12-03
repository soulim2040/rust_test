use crate::devices::Device;

use super::{HasMutDevice, Remote};

pub struct AdvanedDevice<D: Device> {
    device: D,
}

impl<D: Device> AdvanedDevice<D> {
    pub fn new(device: D) -> Self {
        Self {device}
    }

    pub fn mute(&mut self){
        println!("device is muted");
        self.device.set_volume(0);
    }
}

impl<D: Device> HasMutDevice<D> for AdvanedDevice<D> {
    fn device(&mut self) -> &mut D{
        &mut self.device
    }
}

impl<D: Device> Remote<D> for AdvanedDevice<D> {}