
#[derive(Debug, Default)]
pub struct Counter {
    value : usize,
}

pub trait MyIterator{
    type Item;
    fn my_next(&mut self) -> Option<Self::Item>;
}

impl MyIterator for Counter {
    type Item = usize;

    fn my_next(&mut self) -> Option<Self::Item> {
        self.value += 1;
        if self.value < 5 {
            Some(self.value)
        }else{
            None
        }
    }
}