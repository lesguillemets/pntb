use std::ops::Rem;
use std::cmp::PartialEq;

fn gcd<T>(&na: &T, &nb: &T) -> T
where
    T: Rem<T, Output = T> + PartialEq + Copy,
{
    let mut a = na;
    let mut b = nb;
    while b != a {
        // a should b zero
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    println!("{}", gcd(&10, &25));
}
