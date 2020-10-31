use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(mut a: i64, mut b: i64, mut c: i64) -> i64 {
    let mut ans = 0;
    for i in 0..a+1 {
        for j in 0..b+1 {
            for k in 0..c+1 {
                ans += i*j*k;
                ans %= 998244353;
            }
        }
    }
    ans % 998244353
}
