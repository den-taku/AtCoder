use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, s_: String) -> i64 {
    let s: Vec<char> = s_.chars().collect();
    let mut ans = 0;
    for i in 0..n {
        let mut buf = vec![0, 0, 0, 0];
        for j in 0..n-i {
            if s[i+j] == 'A' {
                buf[0] += 1;
            } else if s[i+j] == 'T' {
                buf[1] += 1;
            } else if s[i+j] == 'C' {
                buf[2] += 1;
            } else if s[i+j] == 'G' {
                buf[3] += 1;
            }
            if buf[0] == buf[1] && buf[2] == buf[3] {
                ans += 1;
            }
        }
    }
    ans
}
