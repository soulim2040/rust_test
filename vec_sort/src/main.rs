#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Person {
    name    : String,
    age     : u32,
}

impl Person {
    pub fn new(name : String, age : u32) -> Self {
        Self {
            name, age
        }
    }
}

fn main() {
    // let mut vec = vec![11, 66, 7, 100];
    // vec.sort_unstable_by(|a, b| b.cmp(a));
    // dbg!(vec);

    // let mut fvec = vec![-10.88, 10.0, 0.0, 99_f64, 100.0];
    // fvec.sort_by(|a, b| b.partial_cmp(a).unwrap());
    // dbg!(fvec);

    let mut pvec = vec![
        Person::new("ben".into(), 11),
        Person::new("ken".into(), 33),
        Person::new("ann".into(), 22)
    ];
    pvec.sort();
    dbg!(&pvec);

    pvec.sort_by(|a, b| a.age.cmp(&b.age));
    dbg!(&pvec);
}
