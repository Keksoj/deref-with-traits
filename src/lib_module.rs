use crate::{Running, Walking};
use std::ops::Deref;

pub struct Contest {
    pub name: String,
}

impl Contest {
    pub fn welcome_a_walker(&self, walker: &dyn Walking) {
        println!(
            "The {} championship welcomes a contestant, let's hear him!",
            self.name,
        );
        walker.walk();
    }

    pub fn start_a_race(
        &self,
        // I want to call one Trait but use methods from both
        runner: &dyn Running,
    ) {
        self.welcome_a_walker(runner);
        println!("Welcome! And how fast do you run?");
        runner.run();
    }
}
