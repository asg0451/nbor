#[allow(dead_code)]
mod util {
    pub fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }
}

pub use util::*;
