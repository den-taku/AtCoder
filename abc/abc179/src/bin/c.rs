use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: i64) -> i64 {
    let mut ans = 0;
    for i in 1..n {
            for j in 1..n {
                    if n - i * j > 0 {
                        ans += 1;
                    } else {
                        break;
                    }
            }
    }
    ans
}
