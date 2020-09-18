use competitive::prelude::*;

// better program reads s from the end
#[argio(output = AtCoder)]
fn main(mut s: String) {
    loop {
        if s.starts_with("dreamer") && !s.starts_with("dreamera") {
            s.drain(0..7);
        } else if s.starts_with("dream") {
            s.drain(0..5);
        } else if s.starts_with("eraser") {
            s.drain(0..6);
        } else if s.starts_with("erase") {
            s.drain(0..5);
        } else {
            println!("NO");
            return ();
        }
        if s.len() == 0 {
            println!("YES");
            return ();
        }
    }
}
