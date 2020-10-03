use competitive::prelude::*;
use std::collections::HashSet;

fn make_tuple(n: usize, v: &Vec<i32>, set: &mut HashSet<Vec<i32>>) {
    let mut max = 0;
    let mut ans = Vec::new();
    for i in 0..n {
        if max < v[i] {
            ans.push(-1);
            max = v[i];
        } else {
            ans.push(max);
        }
    }
    set.insert(ans);
}


unsafe fn dfs(n: usize, i: usize, v: Vec<i32>, mut set: &mut HashSet<Vec<i32>>, h: &Vec<i32>) {
    if i == n {
        make_tuple(n, &v, &mut set);
    } else {
        for j in 1..h[i]+1 {
            let mut u = v.clone();
            u.push(j);
            dfs(n, i+1, u, &mut set, &h);
        }
    }
}

#[argio(output = AtCoder)]
fn main(n: usize, h: [i32; n]) -> usize {
    let mut set = HashSet::new();
    unsafe {
        let v: Vec<i32> = Vec::new();
        dfs(n, 0, v, &mut set, &h);
    }



    //

    // ans.iter().fold(0, |max, a| if max >= a { max } else { a });
    set.len() % 1_000_000_007
}
