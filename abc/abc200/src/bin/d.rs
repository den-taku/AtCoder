use proconio::input;

fn main() {
    input! {
            n: usize,
            a: [usize; n]
    }
    let mut v = vec![0;200];
    let mut p = vec![vec![];200];
    for i in 0..n {
        let m = a[i] % 200;
        v[m] += 1;
        p[m].push(i);
    }
    let mut index = vec![];
    for i in 0..200 {
        if v[i] > 2 {
            println!("Yes");
            println!("x {}", p[i][0]);
            println!("y {}", p[i][1]);
            return ();
        }
        if v[i] > 2 {
            index.push(i);
        }
    }
    if v[0] > 0 && index.len() > 1 {
        let mut ans = 0;
        if v[0] == 0 {
            ans = 1;
        }
        println!("Yes");
        println!("x {} {}", p[0][0], an);
        println!("x {}", ans);
    }
    for i in 0..index.len() {
        //
    }
    println!("No");
}