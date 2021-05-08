use proconio::input;

fn main() {
    input! {
            n: i128,
            mut k: i128,
    }
    for i in n..3*n {
        let tmp = cal(n, i);
        if k <= tmp {
            let ans = cal2(n, i, k);
            println!("{} {} {}", ans.0, ans.1, ans.2);
        }
    }

    unimplemented!();
}

fn cal(n: i128, i: i128) -> i128 {
    unimplemented!()
}

fn cal2(n: i128, i: i128, k: i128) -> (i128, i128, i128) {
    unimplemented!()
}