use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: i32, k: i32, m: i32) {
    for x in 1..n+1 {
        let mut ans = 0;
        ans += k;
        let haba = if (x - 1).abs() <= (n - x).abs() { (x - 1).abs() } else { (n - x).abs() };
        // println!("haba: {}", haba);
        if haba != 0 { ans += 1; }
        for i in 1..haba+1 {
            ans *= (k + 1);
            ans %= m;
        }
        if haba != 0 { ans -= 1;}
        println!("{}", ans % m );
    }
}
