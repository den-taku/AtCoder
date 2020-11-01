use competitive::prelude::*;

fn cul(n: usize, v: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in 0..n/2 {
        ans += v[2*i+1] - v[2*i]; 
    }
    ans
}

#[argio(output = AtCoder)]
fn main(n: usize, m: usize, mut h: [i32; n], mut w: [i32; m]) -> i32 {
    let mut com = Vec::new();
    h.sort();
    for i in 0..m {
        let mut buf = h.clone();
        let pos = if let Ok(v) = buf.binary_search(&w[i]) {
            v
        } else if let Err(v) = buf.binary_search(&w[i]) {
            v
        } else {
            0
        };
        buf.insert(pos, w[i].clone());
        com.push(cul(n+1, buf))
    }
    *com.iter().min().unwrap()
}
