use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: i64) -> i64{
    let mut v = vec![];
    for i in 1..20 {
        v.push(((n % (10.0f32.powf(i as f32) as i64)  - (n % (10.0f32.powf((i-1) as f32) as i64))) / (10.0f32.powf((i-1) as f32) as i64)) )
    }
    let mut k = 0;
    for i in 0..19 {
        if v[i] != 0 {
            k += 1;
        }
    }
    v = v.iter().map(|e| e % 3).collect();
    let mut sum = 0;
    for i in 0..19 {
        sum += v[i]
    }
    if sum % 3 == 0 {
        return 0;
    } else if sum % 3 == 1 {
        if v.iter().any(|e| *e == 1) {
            if k > 1 {
                return 1;
            } else {
                return -1;
            }
        } else {
            let mut all = 0;
            for i in 0..19 {
                if v[i] == 2 {
                    all += 1;
                }
            }
            if all >= 2 {
                if k > 2 {
                    return 2;
                } else {
                    return -1;
                }
            } else {
                return -1;
            }
        }
    } else {
        if v.iter().any(|e| *e == 2) {
            if k > 1 {
                return 1;
            } else {
                return -1;
            }
        } else {
            let mut all = 0;
            for i in 0..19 {
                if v[i]  == 1 {
                    all += 1;
                }
            }
            if all >= 2 {
                if k > 2 {
                    return 2;
                } else {
                    return -1;
                }
            } else {
                return -1;
            }
        }
    }
        
}
