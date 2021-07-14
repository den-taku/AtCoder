use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
            mut n: i128,
            k: usize
    }
    for _ in 0..k {
        n = operate(n);
    }
    println!("{}", n);
}

fn operate(n: i128) -> i128 {
    if n % 200 == 0 {
        n / 200
    } else {
        n * 1000 + 200
    }
}