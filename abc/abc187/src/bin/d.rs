use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, v: [(i32, i32); n]) -> i32 {
    let mut ans = 0;
    let mut aoki = v.iter().fold(0, |sum, i| sum + i.0);
    let mut u: Vec<(usize, (i32, i32))> = v.iter().map(|e|{
        (e.0 + e.1, e.0)
    }).enumerate().collect();
    let r = u.clone();
    u.sort_by(|a, b| a.1.cmp(&b.1));
    while aoki >= 0 {
        let index = u.pop().unwrap().0;
        aoki -= r[index].1.0 as i32;
        aoki -= v[index].0;
        ans += 1;
    }

    ans
}
