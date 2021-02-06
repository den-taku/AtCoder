use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(v: i32, t: i32, s: i32, d: i32) {
    if v * t <= d && d <= v * s {
        println!("No")
    } else {
        println!("Yes")
    }
}
