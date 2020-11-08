use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, mut a: [i64; n]) -> i64 {
    let mut ans = 2;
    let mut buf = 0;
    a.sort();
    let max = a[n-1];
    for i in 2..max+1 {
        let mut com = 0;
        for j in 0..n {
            if a[j] % i == 0 {
                com += 1;
            }
        }
        if buf <= com {
            buf = com;
            ans = i
        }
    }
    ans
}
