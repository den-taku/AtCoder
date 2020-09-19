use competitive::prelude::*;
use std::cmp::max;

fn sum_i(l: usize, r_v: &Vec<i64>, i: usize, k: usize) -> Vec<i64> {
    let mut sum = Vec::new();
    (0..k).collect::<Vec<usize>>().iter().fold(i, |tag, x| {
        //sum += r_v[tag];
        if *x == 0 {
            sum.push(r_v[tag]);
        } else {
            sum.push(r_v[tag] + sum[x - 1]);
        }
        // println!("l: {}, tag: {}", l, tag);
        if tag+1 == l {
            tag + 1 - l
        } else {
            tag + 1
        }
    });
    sum
}

#[argio(output = AtCoder)]
fn main(n: usize, k: i64, mut point: [usize; n], score: [i64; n]) -> i64 {
    // make usize loops from point
    let k = k as usize;
    let mut v: Vec<Vec<i64>> = Vec::new();
    point = point.iter().map(|e| { e - 1 } ).collect();
    for i in 0..n {
        if point[i] != n {
            let mut tmp = Vec::new();
            let mut j = i;
            while point[j] != n {
                tmp.push(score[j]);
                let k = point[j];
                point[j] = n;
                j = k;
            }
            v.push(tmp);
        }
    }
    // println!("{:?}", v);
    v.iter()
        .fold(std::i64::MIN, |ans, w| { 
            let w_ln = w.len();
            let mut dp = vec![std::i64::MIN; k];
            for i in 0..w_ln {
                // dp[j] = max(dp[j], sum_i(w_ln, &w, j, i))
                let buf = sum_i(w_ln, &w, i, k);
                for j in 0..k {
                    dp[j] = max(dp[j], buf[j]);
                }
            }
            // println!("{:?}", dp);
            let max_i = dp.iter().fold(std::i64::MIN, |m, e| { max(m, *e) });
            // let max_i = w.iter()
            //     .cycle()
            //     .fold(0, |i, e| );
            if max_i >= ans {
                max_i
            } else {
                ans
            }
        })
}
