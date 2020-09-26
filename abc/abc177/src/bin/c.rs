use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, mut a: [i64;n]) -> i64 {
    let mut ans = 0;
    a = a.iter().map(|num| num % 1_000_000_007).collect();
    let mut sum = a.iter().fold(0, |s, num| s + num) /*% 1_000_000_007*/;
    for i in 0..n {
        sum -= a[i];
        ans += a[i] * sum /*% 1_000_000_007*/;
    }
    ans % 1_000_000_007
}
