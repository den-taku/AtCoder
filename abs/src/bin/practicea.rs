use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(a: [i64; 3], s: String) {
    println!("{} {}", a[0] + a[1] + a[2], s);
}
