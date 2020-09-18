use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(s: i32) -> i32 {
    match s {
        000 => 0,
        001 => 1,
        010 => 1,
        100 => 1,
        011 => 2,
        101 => 2,
        110 => 2,
        111 => 3,
         _  => 100,
    }
}
