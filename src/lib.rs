use std::ops::Deref;

pub mod lib_module;

pub trait Walking {
    fn walk(&self);
}

pub trait Running: Walking {
    fn run(&self);
}

impl<T: Deref<Target = U>, U: Walking> Walking for T {
    fn walk(&self) {
        self.deref().walk()
    }
}
