use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(a: [i64; 4]) -> i64 {
    let mut vec = Vec::new();
    vec.push(a[0] * a[2]);
    vec.push(a[0] * a[3]);
    vec.push(a[1] * a[2]);
    vec.push(a[1] * a[3]);
    if let Some(num) = vec.iter().max() {
        return *num;
    }
    0
}
