
use super::{Department, Patient, into_next};
pub struct Medical{
    next : Option<Box<dyn Department>>,
}

impl Medical{
    pub fn new(next :impl Department + 'static) -> Self{
        Self {
            next : into_next(next),
        }
    }
}

impl Department for Medical {
    fn handle(&mut self, patient: &mut Patient){
        if patient.medical_get {
            println!("medical has get");
        }else{
            println!("medical giving to patient {}", patient.name);
            patient.medical_get = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>>{
        &mut self.next
    }
}