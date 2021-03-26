use moving::{Running, Walking};
use std::ops::Deref;

pub struct Walker {
    pub name: String,
}

impl Walking for Walker {
    fn walk(&self) {
        println!("I can walk, my name is {}", self.name);
    }
}

pub struct Runner {
    pub walker_with_name: Walker,
    pub sprint_time: f32,
}

impl Running for Runner {
    fn run(&self) {
        println!("I run 100 meters in {} seconds!", self.sprint_time);
    }
    fn walk(&self){
        self.deref().walk()
    }
}

impl Deref for Runner {
    type Target = Walker;
    fn deref(&self) -> &Self::Target {
        &self.walker_with_name
    }
}
