extern crate structure;
// use structure::foo; private
use structure::bar::Bar;

fn main() {
    structure::hello();
    structure::say_foo(32);
    // println!("{:?}", structure::Bar::new(2)); private
    println!("{:?}", structure::bar::Bar::new(2));
    println!("{:?}", Bar::new(23));
}
