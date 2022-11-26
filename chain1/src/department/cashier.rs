
use super::{Department, Patient, into_next};
#[derive(Default)]
pub struct Cashier{
    next : Option<Box<dyn Department>>,
}

impl Department for Cashier {
    fn handle(&mut self, patient: &mut Patient){
        if patient.payed {
            println!("money has payed");
        }else{
            println!("cashier gettting money from patient {}", patient.name);
            patient.payed = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>>{
        &mut self.next
    }
}
