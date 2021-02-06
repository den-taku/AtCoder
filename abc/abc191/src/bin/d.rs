use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(x: f32, y: f32, r: f32) -> i32 {
    let low = (x - r).ceil() as i32;
    let high = (x + r).floor() as i32;

    let mut ans = 0;

    for i in low..high + 1 {
        let p = (r.powf(2.0) - (x - i as f32).powf(2.0)).sqrt();
        let bottom = (y - p).ceil() as i32;
        let top = (y + p).floor() as i32;
        for _ in bottom..top + 1 {
            ans += 1
        }
    }
    ans
}
