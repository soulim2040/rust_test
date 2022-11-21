mod user;

use once_cell::sync::OnceCell;
use std::sync::Mutex;
use user::User;


static ARRAY : OnceCell<Mutex<Vec<i32>>> = OnceCell::new();

fn singleton_init(vec : Vec<i32>){
    ARRAY.get_or_init(|| Mutex::new(vec));
}

fn vec_push(it : i32) {
    ARRAY.get().unwrap().lock().unwrap().push(it);
}

fn main() {
    singleton_init(Vec::new());
    vec_push(11);

    dbg!(ARRAY.get().unwrap().lock().unwrap());

    let user1 = User::new("ben".into(), "farmer".into());
    println!("{}, {}", user1.name(), user1.surname());
}
