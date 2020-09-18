use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, y: i32) {
    for i in 0..n+1 {
        for j in 0..n-i+1 {
            if n as i32 - i as i32 - j as i32== y / 1000 - 10 * i as i32- 5 * j as i32 {
                println!("{} {} {}", i, j, n - i - j);
                std::process::exit(0);
            }
        }
    }
    println!("{} {} {}", -1, -1, -1);
}
