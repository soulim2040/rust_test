mod user;
mod counter;

use user::UserVec;
use counter::Counter;
use counter::MyIterator;

fn main() {
    // let vec = vec![11, 33, 44];
    // vec.iter().for_each(|s| println!("{}", s));

    // let user = UserVec::new();
    // user.iter().for_each(|s| println!("{}", s));

    let mut count = Counter::default();
    while let Some(cnt) = count.my_next() {
        println!("{}", cnt);
    }
}
