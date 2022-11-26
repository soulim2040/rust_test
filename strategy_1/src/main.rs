trait RouteStrategy {
    fn build_route(&self, from : &str, to : &str);
}

struct WalkRoute;
impl RouteStrategy for WalkRoute {
    fn build_route(&self, from : &str, to : &str) {
        println!("walk route from {} to {}", from, to);
    }
}

struct CarRoute;
impl RouteStrategy for CarRoute {
    fn build_route(&self, from : &str, to : &str) {
        println!("car route from {} to {}", from, to);
    }
}

struct RouteBuilder<T : RouteStrategy>{
    route_strategy : T,
}

impl<T : RouteStrategy> RouteBuilder<T> {
    pub fn new(route_strategy : T) -> Self {
        Self {route_strategy}
    }

    pub fn route(&self, from : &str, to : &str){
        self.route_strategy.build_route(from, to);
    }
}


// ---------------------------------------------------------------
// type RouteFn = fn(from : &str, to : &str);

// fn walk_route_fn(from : &str, to : &str){
//     println!("walk route fn from {} to {}", from, to);
// }

// fn car_route_fn(from : &str, to : &str){
//     println!("car route fn from {} to {}", from, to);
// }

// struct RouteFnBuilder {
//     route_fn : RouteFn,
// }

// impl RouteFnBuilder{
//     pub fn new(route_fn : RouteFn) -> Self{
//         Self { route_fn }
//     }

//     pub fn route(&self, from : &str, to : &str){
//         (self.route_fn)(from, to);   
//     }
// }
// ---------------------------------------------------------------

type RouteStrategyFn = fn(from: &str, to: &str);

fn walking_strategy(from: &str, to: &str) {
    println!("Walking route from {} to {}: 4 km, 30 min", from, to);
}

fn public_transport_strategy(from: &str, to: &str) {
    println!(
        "Public transport route from {} to {}: 3 km, 5 min",
        from, to
    );
}

struct Navigator {
    route_strategy: RouteStrategyFn,
}

impl Navigator {
    pub fn new(route_strategy: RouteStrategyFn) -> Self {
        Self { route_strategy }
    }

    pub fn route(&self, from: &str, to: &str) {
        self.route_strategy(from, to);
    }
}

fn main() {
    let route_builder = RouteBuilder::new(WalkRoute);
    route_builder.route("home".into(), "company".into());

    let car_builder = RouteBuilder::new(CarRoute);
    car_builder.route("home".into(), "company".into());

    // let route_fn = RouteFnBuilder::new(walk_route_fn);
    // route_fn.route("park".into(), "toilet".into());

    let navigator = Navigator::new(|from, to| println!("Specific route from {} to {}", from, to));
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");
}
