use std::fmt::Display;

pub trait Tty {
    fn print<T: Display+?Sized>(&self, item: &T) {
        println!("{}", item);
    }
}
