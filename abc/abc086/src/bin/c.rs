use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, p: [(i32, i32, i32); n]) -> bool {
    if p[0].0 < p[0].1.abs() + p[0].2.abs() || p[0].0 % 2 != (p[0].1.abs() + p[0].2.abs()) % 2 {
        return false;
    }
    for i in 1..n {
        let dt = p[i].0 - p[i-1].0;
        let dx = (p[i].1 - p[i-1].1).abs();
        let dy = (p[i].2 - p[i-1].2).abs();
        if !( dx + dy <= dt) {
            return false;
        }
        if dt % 2 != (dx + dy) % 2 {
            return false;
        }
    }
    true
}
