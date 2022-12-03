use crate::devices::Device;

use super::{HasMutDevice, Remote};

pub struct BasicDevice<D: Device> {
    device: D,
}

impl<D: Device> BasicDevice<D> {
    pub fn new(device: D) -> Self {
        Self {device}
    }
}

impl<D: Device> HasMutDevice<D> for BasicDevice<D> {
    fn device(&mut self) -> &mut D{
        &mut self.device
    }
}
impl<D: Device> Remote<D> for BasicDevice<D> {}