#[derive(Debug)]
pub struct Foo {
    foo: u32,
}

impl Foo {
    pub fn new(f: u32) -> Self {
        Foo { foo: f }
    }
}
