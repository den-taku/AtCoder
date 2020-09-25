use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(mut s: String) -> String {
    if s.ends_with('s') {
            s.push_str("es");
            s
    } else {
            s.push('s');
            s
    }
}
