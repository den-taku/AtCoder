use competitive::prelude::*;

fn check(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
    b.0*c.1 - a.0*c.1 == b.1*c.0 - a.1*c.0 + a.1*b.0 - b.1*a.0
}

#[argio(output = AtCoder)]
fn main(n: usize, x: [(i64, i64); n]) -> bool {
    for i in 0..n-2 {
        for j in i+1..n-1 {
            for k in j+1..n {
                if check(x[i].clone(), x[j].clone(), x[k].clone()) {
                    // println!("{:?} {:?} {:?}", x[i].clone(), x[j].clone(), x[k].clone());
                    return true;
                }
            }
        }
    }
    false
}
