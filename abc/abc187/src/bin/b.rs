use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, pos: [(i32, i32); n]) -> i32 {
    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
           let err = (pos[i].0 - pos[j].0).abs();
           if (pos[j].1 <= pos[i].1 + err) && (pos[j].1 >= pos[i].1 - err) {
                ans += 1;
            }
        }
    }
    ans
}
