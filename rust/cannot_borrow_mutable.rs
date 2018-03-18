/// Most likely https://github.com/rust-lang/rfcs/pull/2094
fn main() {
    let mut v = vec![0, 1, 2];
    {
        let c = normal(&mut v, 0);
        *c = 10;
    }
    println!("{:?}", v);
    ()
}

fn normal<T>(vs: &mut Vec<T>, n: usize) -> &mut T {
    &mut vs[n]
}

fn error<T>(vs: &mut Vec<T>, n: usize) -> &mut T {
    if let Some(c) = vs.get_mut(n) {
        c
    } else {
        &mut vs[0]
    }
}
