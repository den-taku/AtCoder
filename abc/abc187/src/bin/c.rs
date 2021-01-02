use competitive::prelude::*;
use std::collections::HashSet;

#[argio(output = AtCoder)]
fn main(n: usize, s: [String; n]) {
    let mut hash0 = HashSet::new();
    let mut hash1 = HashSet::new();
    let mut flag = true;
    for i in 0..n {
        if let Some(_) = s[i].find('!') {
            hash1.insert(s[i].trim_matches('!'));
            if hash0.contains((s[i].clone().trim_matches('!'))) {
                println!("{}", s[i].trim_matches('!'));
                flag = false;
                break;
            }
        } else {
            hash0.insert(s[i].clone());
            if hash1.contains(s[i].clone().trim_matches('!')) {
                println!("{}", s[i]);
                flag = false;
                break;
            }
        }
    }
    if flag {
    println!("satisfiable"); 
    }
}
