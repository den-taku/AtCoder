use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: isize,
    }
    println!("{}", (n-1) / 100 + 1);
}
