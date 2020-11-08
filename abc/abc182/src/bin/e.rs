use competitive::prelude::*;

fn update(mut field: Vec<i32>, i: usize, j: usize, h: usize, w: usize) -> Vec<i32> {
    let mut count = 0;
    field[i*w + j] = 1;
    while i + count < h {
        if field[(i + count) * w + j] != -1 {
            field[(i + count) * w + j] = 1;
        } else {
            break;
        }
        count += 1;
    }
    count = 0;
    while i as isize - count as isize >= 0 {
        if field[(i - count) * w + j] != -1 {
            field[(i - count) * w + j] = 1;
        } else {
            break;
        }
        count += 1;
    }
    count = 0;
    while j + count < w {
        if field[i * w + j + count] != -1 {
            field[i * w + j + count] = 1;
        } else {
            break;
        }
        count += 1;
    }
    count = 0;
    while j as isize - count as isize >= 0 {
        if field[i * w + j - count] != -1 {
            field[i * w + j - count] = 1;
        } else {
            break;
        }
        count += 1;
    }
    field
}

#[argio(output = AtCoder)]
fn main(h: usize, w: usize, n: usize, m: usize, mut d: [(usize, usize); n], mut b: [(usize, usize); m]) -> i32 {
    d = d.iter().map(|e| (e.0 - 1, e.1 - 1)).collect();
    b = b.iter().map(|e| (e.0 - 1, e.1 - 1)).collect();
    let mut field = vec![0i32; h*w];
    for i in 0..m {
        field[b[i].0 * w + b[i].1] = -1;
    }
    for i in 0..n {
        field = update(field, d[i].0, d[i].1, h, w);
    }
    let mut ans = 0;
    for i in 0..h*w {
        if field[i] == 1 {
            ans += 1;
        }
    }
    ans
}
