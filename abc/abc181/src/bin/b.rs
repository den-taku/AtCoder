use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, v: [(i64, i64); n]) -> i64 {
    let mut ans = 0;
    for i in 0..n {
        ans += (v[i].0 + v[i].1) * (v[i].1 - v[i].0 + 1) / 2;
    }
    ans
}
