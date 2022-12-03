use super::Component;

pub struct Folder{
    name: &'static str,
    components: Vec<Box<dyn Component>>,
}

impl Folder{
    pub fn new(name: &'static str) -> Self{
        Self{ 
            name : name,
            components: vec![],
        }
    }
    
    pub fn add(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }
}

impl Component for Folder{
    fn search(&self, keyword: &str) {
        println!("searching keyword {} in folder {}", keyword, self.name);

        for com in self.components.iter() {
            com.search(keyword);
        }
    }
}