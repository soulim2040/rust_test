mod cashier;
mod medical;
mod doctor;
mod reception;

use crate::patient::Patient;

pub use cashier::Cashier;
pub use medical::Medical;
pub use doctor::Doctor;
pub use reception::Reception;

pub trait Department {
    fn execute(&mut self, patient: &mut Patient){
        self.handle(patient);

        if let Some(department) = self.next() {
            department.execute(patient);
        } 
    }
    fn handle(&mut self, patient: &mut Patient);
    fn next(&mut self) ->&mut Option<Box<dyn Department>>;
}

pub fn into_next(department: impl Department + Sized + 'static) 
    -> Option<Box<dyn Department>> {
        Some(Box::new(department))
    }