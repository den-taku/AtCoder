use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, a: [i64; n]) -> i64 {
    let mut ans = 0;
    let mut pos = 0;
    for i in 0..n {
        for j in 0..i+1 {
            pos += a[j];
            ans = if ans >= pos { ans } else { pos };
        }
    }
    ans
}
