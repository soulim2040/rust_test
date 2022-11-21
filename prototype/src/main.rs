
#[derive(Clone, Debug)]
pub struct Point {
    x : f32,
    y : f32,
}

fn main() {
    let p1 = Point { x : 1.0, y : 1.0};
    let mut p2 = p1.clone();
    p2.x = 2.0;
    p2.y = 2.0;
    println!("{:?}", p1);
    println!("{:?}", p2);
}
