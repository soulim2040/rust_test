
use super::{Department, Patient, into_next};
pub struct Reception{
    next : Option<Box<dyn Department>>,
}

impl Reception{
    pub fn new(next :impl Department + 'static) -> Self{
        Self {
            next : into_next(next),
        }
    }
}
impl Department for Reception {
    fn handle(&mut self, patient: &mut Patient){
        if patient.registered {
            println!("patient already registered");
        }else{
            println!("patient gettting registere {}", patient.name);
            patient.registered = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>>{
        &mut self.next
    }
}