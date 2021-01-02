use competitive::prelude::*;

fn sum(a: i32) -> i32 {
    let one = a / 100;
    let two = a / 10 - one * 10;
    let three = a - one * 100 - two * 10;
    one + two + three
}

#[argio(output = AtCoder)]
fn main(a: i32, b: i32) -> i32 {
    let sum_a = sum(a);
    let sum_b = sum(b);
    if sum_a > sum_b {
        sum_a
    } else {
        sum_b
    }
}
