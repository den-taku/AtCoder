use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(N: usize, mut A: [i64; N]) -> i64 {
    let mut ans = 0;
    while A.iter().all(|i| i % 2 == 0) {
        ans += 1;
        A = A.iter().map(|i| i / 2).collect();
    }
    ans
}
