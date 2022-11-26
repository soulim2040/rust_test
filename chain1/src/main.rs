mod department;
mod patient;

use crate::patient::Patient;
use crate::department::{Department, Cashier, Medical, Doctor, Reception};

fn main() {
    let cachier = Cashier::default();
    let medical = Medical::new(cachier);
    let doctor = Doctor::new(medical);
    let mut reception = Reception::new(doctor);
    
    let mut patient = Patient {
        name : "LiLei".into(),
        .. Patient::default()
    };

    reception.execute(&mut patient);
}
