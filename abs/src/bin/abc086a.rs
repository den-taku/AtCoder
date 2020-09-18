use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(a: i64, b: i64) {
    if a % 2 == 0 || b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
