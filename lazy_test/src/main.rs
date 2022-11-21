use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static!{
    static ref ARRAY : Mutex<Vec<i32>> = Mutex::new(vec!());
}

fn main() {
    ARRAY.lock().unwrap().push(11);

    dbg!(ARRAY.lock().unwrap());
}
