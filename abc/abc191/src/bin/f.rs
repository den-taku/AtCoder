use competitive::prelude::*;
use std::collections::BTreeSet;

fn gcd_set(a: Vec<usize>) -> BTreeSet<usize> {
    let mut set = BTreeSet::new();
    for i in 0..a.len() {
        for j in i+1..a.len() {
            set.insert(gcd(a[i], a[j]));
        }
    }
    set
}

#[argio(output = AtCoder)]
fn main(n: usize, mut a: [usize; n]) -> usize {
    let set = gcd_set(a.clone());
    a.sort_by_key(|&x| Reverse(x));
    if *set.iter().last().unwrap() == a.pop().unwrap() {
        set.len() + 1
    } else {
        set.len() + 2
    }
}
