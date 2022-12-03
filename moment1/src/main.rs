trait moment<T> {
    fn restore(&self) -> T;
    fn print(&self);
}

struct Originator{
    state: u32,
}

impl Originator {
    fn save(&self) -> OriginatorBackup {
        OriginatorBackup{
            state: self.state.to_string(),
        }
    }
}

struct OriginatorBackup {
    state: String,
}

impl moment<Originator> for OriginatorBackup{
    fn restore(&self) -> Originator{
        Originator{
            state: self.state.parse().unwrap(),
        }
    }

    fn print(&self){
        println!("OriginatorBackup state is {}", self.state);
    }
}
fn main() {
    let mut history = Vec::<OriginatorBackup>::new();

    let mut originator = Originator{state : 0};

    history.push(originator.save());

    originator.state = 1;
    history.push(originator.save());

    for mom in history.iter() {
        mom.print();
    }

    let restore1 = history.pop().unwrap().restore();
    println!("restore state {}", restore1.state);

    for mom in history.iter() {
        mom.print();
    }
}
