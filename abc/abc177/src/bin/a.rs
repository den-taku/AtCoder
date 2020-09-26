use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(d: f32, t: f32, s: f32) -> bool {
    d / s <= t
}
