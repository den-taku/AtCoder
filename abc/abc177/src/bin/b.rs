use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(s: String, t: String) -> usize {
    let mut ans = 0;
    let rs = s.into_bytes();
    let rt = t.into_bytes();
    for i in 0..rs.len()-rt.len()+1{
        let mut count = 0;
        for j in 0..rt.len() {
            if rs[i + j] == rt[j] {
                count += 1;
            }
        }
        if ans < count {
            ans = count;
        }
    }
    if rt.len() - ans > 0 {
        rt.len() - ans 
    } else {
        0
    }
}
