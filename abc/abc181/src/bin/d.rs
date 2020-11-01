use competitive::prelude::*;

fn check(a: i64, b: i64, c: i64) -> bool {
    // println!("{} {} {}", a, b, c);
    if a == 0 || b == 0 || c == 0 {
        false
    } else {
    (a * 100 + b * 10 + c) % 8 == 0
    || (a * 100 + c * 10 + b) % 8 == 0
    || (b * 100 + a * 10 + c) % 8 == 0
    || (b * 100 + c * 10 + a) % 8 == 0
    || (c * 100 + a * 10 + b) % 8 == 0
    || (c * 100 + b * 10 + a) % 8 == 0
    }
}
fn check1(c: i64) -> bool {
    // println!("{} {} {}", a, b, c);
    c % 8 == 0
}
fn check2(b: i64, c: i64) -> bool {
    // println!("{} {} {}", a, b, c);
    (b * 10 + c) % 8 == 0
    || (c * 10 + b) % 8 == 0
}
#[argio(output = AtCoder)]
fn main(s: i64) -> bool {
    let v = vec![(s / 10000), (s / 1000) - 10*(s / 10000), 
        (s / 100) - 10 * (s / 1000), (s / 10) - 10 * (s / 100), s - 10 * (s / 10)];
    // println!("{:?}", v);
    if check1(v[4].clone()) {
        return true;
    }
    if check2(v[3].clone(), v[4].clone()) {
        return true;
    }
    for i in 0..4 {
        for j in i+1..4 {
            for k in j+1..5 {
                // println!("{}{}{}", i, j, k);
                if check(v[i].clone(), v[j].clone(), v[k].clone()) {
                    return true;
                } 

            }
        }
    }
    false
}
