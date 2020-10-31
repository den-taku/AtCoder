use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: i64, K: i64) -> i64{
    let mut ans = 0;
    'out:
    for i in 1..n+1 {
        for j in 1..n+1 {
            if i + j -n -K >= n+1 {
                continue 'out;
            }
            for k in 1..n+1 {
                if i+j-k-K >= 1 && i+j-k-K <= n {
                    ans += 1;
                }
            }
        }
    }
    ans 
}
