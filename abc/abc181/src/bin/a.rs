use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: i32) {
    if n % 2 == 0 {
        println!("White")
    } else {
        println!("Black");
    }
}
