use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(N: i64, A: i64, B: i64) -> i64 {
    let mut ans = 0;
    for i in 1..N+1 {
        let s1 = i % 10;
        if s1 <= B {
            let s2 = ((i - s1) % 100)/10 + s1;
            if s2 <= B {
                let s3 = ((i - s2) % 1000)/100 + s2;
                if s3 <= B {
                    let s4 = ((i - s3) % 10000)/1000 + s3;
                    if s4 <= B {
                        let s5 = ((i - s4) % 100000)/10000 + s4;
                        if A<= s5 && s5 <= B {
                            ans += i;
                        }
                    }
                }
            }
        }
    }
    ans
}
