use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, d: [u8; 2 * n]) -> bool {
    for i in 0..n-2 {
        if d[2 * i] == d[2 * i + 1] {
            if d[2 * (i + 1)] == d[2 * (i + 1) + 1] {
                if d[2 * (i + 2)] == d[2 * (i + 2) + 1] {
                    return true;
                }
            }
        }
    }
    false
}
