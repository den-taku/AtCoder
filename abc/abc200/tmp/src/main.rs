use proconio::input;

fn main() {
    input! {
            n: usize,
            a: [i128; n]
    }
    let mut v: Vec<i128> = vec![0;200];
    for i in 0..n {
        let m = a[i] % 200;
        v[m as usize] += 1;
    }
    let mut ans = 0;
    for i in 0..200 {
        ans += v[i] * (v[i] - 1) / 2; 
    }
    println!("{}", ans);
}