pub mod app;

pub trait Inverse {
    fn inverse(&mut self);
}

impl Inverse for bool {
    fn inverse(&mut self) {
        *self = !*self;
    }
}
