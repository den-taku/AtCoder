use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
            n: usize,
            a: [usize; n]
    }
    let mut v = vec![0;200];
    for i in 0..n {
        let m = a[i] % 200;
        v[m] += 1;
    }
    let mut ans = 0;
    for i in 0..200 {
        if v[i] != 0 {
            ans += v[i] * (v[i] - 1) / 2; 
        }
    }
    println!("{}", ans);
}