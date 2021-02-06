use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, x: i32, a: [i32; n]) {
    let mut v = Vec::new();
    for i in 0..n {
        if a[i] != x {
            v.push(a[i].clone())
        }
    }
    for e in &v {
        print!("{} ", e)
    }
}
