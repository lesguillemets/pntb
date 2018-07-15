#[derive(Debug)]
pub struct Bar {
    bar: u32,
}

impl Bar {
    pub fn new(b: u32) -> Self {
        Bar { bar: b }
    }
}
