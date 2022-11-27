
pub trait Visitor {
    type Value;
    fn visit_vec(&self, v: Vec<i32>) -> Self::Value;
}

pub trait Deserialzer<V: Visitor> {
    fn create(visitor: V) -> Self;

    fn parse_str(&self, input: &str) -> Result<V::Value, &'static str>{
        Err("parse_str is not implemented")
    }

    fn parse_vec(&self, input: Vec<i32>) -> Result<V::Value, &'static str>{
        Err("parse_vec is not implemented")
    }
}