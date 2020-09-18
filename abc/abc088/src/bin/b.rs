use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, mut a: [i64; n]) -> i64 {
    a.sort();
    a.reverse();
    let mut a_p = 0;
    let mut b_p   = 0;
    a.iter().fold(1, |i, num| {
        if i % 2 == 1 {
            a_p += num;
        } else {
            b_p += num;
        }
        i + 1
        });
    a_p - b_p
}
