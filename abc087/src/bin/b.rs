use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(A: i64, B: i64, C: i64, X: i64) -> i64 {
    let mut ans = 0;
    for a in 0..A+1 {
        for b in 0..B+1 {
            for c in 0..C+1 {
                if X == 500 * a + 100 * b + 50 * c {
                    ans += 1;
                }
            }
        }
    }
    ans
}
