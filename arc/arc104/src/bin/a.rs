use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(a: i64, b: i64) {
    println!("{}", (a + b) / 2);
    println!("{}", (a - b) / 2);
}
