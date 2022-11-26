
use super::{Department, Patient, into_next};
pub struct Doctor{
    next : Option<Box<dyn Department>>,
}

impl Doctor{
    pub fn new(next :impl Department + 'static) -> Self{
        Self {
            next : into_next(next),
        }
    }
}

impl Department for Doctor {
    fn handle(&mut self, patient: &mut Patient){
        if patient.doctor_checked {
            println!("doctor has checked");
        }else{
            println!("doctor checking patient {}", patient.name);
            patient.doctor_checked = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>>{
        &mut self.next
    }
}