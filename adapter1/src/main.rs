
trait Target {
    fn request(&self) -> String;
}

struct OrdinaryTarget;

impl Target for OrdinaryTarget {
    fn request(&self) -> String {
        "ordinary request".into()
    }
}

struct SpecificTarget;

impl Target for SpecificTarget {
    fn request(&self) -> String {
        "tseuqer cificepSAA".into()
    }
}

struct TargetAdapter {
    spec_target: SpecificTarget,
}

impl Target for TargetAdapter {
    fn request(&self) -> String {
        self.spec_target.request().chars().rev().collect()
    }
}
fn call(target: impl Target){
    println!("call {}", target.request());
}

fn main() {
    let specific_target = SpecificTarget;
    let target = TargetAdapter{ spec_target: specific_target};
    call(target);
}
