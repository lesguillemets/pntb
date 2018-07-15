pub mod bar;
mod foo;
use bar::Bar;
use foo::Foo;

pub fn hello() {
    println!("HI");
}

pub fn say_foo(f: u32) -> () {
    println!("{:?}", Foo::new(f));
}
