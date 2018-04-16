use std::io;
use std::io::stdin;
use std::io::BufRead;
use std::ops::Neg;
use std::iter;

fn main() {
    println!("{}", f());
}

fn f() -> u32 {
    let mut xss = &[1, 2, 3];
    fn g(xs: &mut Iterator<Item = &u32>) -> u32 {
        if let Some(x) = xs.next() {
            x + g(&mut xs)
        } else {
            0
        }
    }
    g(&mut xss.into_iter())
}
