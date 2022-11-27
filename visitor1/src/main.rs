mod visitor;

use crate::visitor::{Visitor, Deserialzer};

#[derive(Debug,Default)]
pub struct TwoValuesStruct {
    a: i32,
    b: i32,
}

impl Visitor for TwoValuesStruct {
    type Value = TwoValuesStruct;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value{
        TwoValuesStruct{
            a: v[0],
            b: v[1],
        }
    }
}

#[derive(Debug,Default)]
pub struct TwoValuesArray {
    ab: [i32; 2],
}

impl Visitor for TwoValuesArray {
    type Value = TwoValuesArray;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value{
        let mut ab = [0i32; 2];
        ab[0] = v[0];
        ab[1] = v[1];
        TwoValuesArray{
            ab: ab,
        }
    }
}

#[derive(Debug)]
struct StringDeserializer<V: Visitor> {
    visitor: V,
}

impl<V: Visitor> Deserialzer<V> for StringDeserializer<V>{
    fn create(visitor: V) -> Self{
        Self { visitor}
    }

    fn parse_str(&self, input: &str) -> Result<V::Value, &'static str>{
        let input_vec: Vec<i32> = input
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Ok(self.visitor.visit_vec(input_vec))
    }
}

struct VecDeserializer<V: Visitor> {
    visitor: V,
}

impl<V: Visitor> Deserialzer<V> for VecDeserializer<V>{
    fn create(visitor: V) -> Self{
        Self { visitor}
    }

    fn parse_vec(&self, input: Vec<i32>) -> Result<V::Value, &'static str>{
        Ok(self.visitor.visit_vec(input))
    }
}
fn main() {
    // parse from str
    let deserialzer = StringDeserializer::create(TwoValuesArray::default());
    let val = deserialzer.parse_str("11 22").unwrap();
    dbg!(val);

    let deserialzer2 = StringDeserializer::create(TwoValuesStruct::default());
    let val2 = deserialzer2.parse_str("11 22").unwrap();
    dbg!(val2);

    // parse from vec
    let deserialzer3 = VecDeserializer::create(TwoValuesArray::default());
    let val3 = deserialzer3.parse_vec(vec![33, 44]).unwrap();
    dbg!(val3);

    let deserialzer4 = VecDeserializer::create(TwoValuesStruct::default());
    let val4 = deserialzer4.parse_vec(vec![33, 44]).unwrap();
    dbg!(val4);
}
