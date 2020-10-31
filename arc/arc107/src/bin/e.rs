use competitive::prelude::*;

fn mex(i: i8, j: i8) -> i8 {
    if i == 0 {
        if j == 1 {
            2i8
        } else {
            1i8
        }
    } else if i == 1 {
        if j == 0 {
            2i8 
        } else {
            0i8
        }
    } else {
        if j == 0 {
            1i8
        } else {
            0i8
        }
    }
}

#[argio(output = AtCoder)]
fn main(n: usize, l: [i8; n], c: [i8; n-1]) {
    let mut ans0 = 0;
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut a = vec![0i8; n*n];
    for i in 0..n {
        a[i] = l[i].clone();
        if l[i] == 1 {
            ans1 += 1;
        } else if l[i] == 0 {
            ans0 += 1;
        } else {
            ans2 += 1;
        }
    }
    for j in 0..n-1 {
        a[(j+1)*n] = c[j].clone();
        if c[j] == 1 {
            ans1 += 1;
        } else if c[j] == 0 {
            ans0 += 1;
        } else {
            ans2 += 1;
        }
    }
    for i in 1..n {
        for j in 1..n {
            a[i * n + j] = mex(a[(i-1)*n+j].clone(), a[i*n+j-1].clone());
            if a[i*n+j] == 0 {
                ans0 += 1;
            } else if a[i*n+j] == 1 {
                ans1 += 1;
            } else {
                ans2 += 1;
            }
        }
    }
    // println!("{:?}", a);
    println!("{} {} {}", ans0, ans1, ans2);
}
