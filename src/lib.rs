pub mod lib_module;

pub trait Walking {
    fn walk(&self);
}

pub trait Running{
    fn run(&self);
    fn walk(&self);

}
