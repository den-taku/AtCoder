use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, mut d: [i32; n]) -> i32 {
    d.sort();
    let mut ans = 0;
    let mut tmp = -1000;
    d = d.iter()
        .filter(|num| { if tmp == **num { false } else { tmp = **num; true }})
        .fold(vec![], |_,_| { ans += 1; vec![]});
    ans
}
