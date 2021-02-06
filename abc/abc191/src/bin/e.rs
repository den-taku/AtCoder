use competitive::prelude::*;

fn check_self(load: Vec<(usize, usize, usize)>, n: usize) -> (Vec<Vec<Option<usize>>>, Vec<usize>) {
    let mut non_self = vec![vec![None; n]; n];
    let mut selfs = vec![std::usize::MAX; n];
    for e in load {
        if e.0 == e.1 {
            if selfs[e.0- 1] > e.2 {
                selfs[e.0- 1] = e.2;
            }
        } else {
            if let Some(w) = non_self[e.0- 1][e.1- 1] {
                if w > e.2 {
                    non_self[e.0- 1][e.1- 1] = Some(e.2);
                }
            } else {
                non_self[e.0- 1][e.1- 1] = Some(e.2);
            }
        }
    }
    (non_self, selfs)
}

fn warshall_floyd(dist: &mut Vec<Vec<Option<usize>>>) -> Vec<Vec<usize>> {
    let n = dist.len();
    let mut next = vec![];
    // next[i][j] = j で初期化
    for i in 0..n {
        next.push((0..n).map(|j| j).collect::<Vec<usize>>());
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(dik), Some(dkj)) = (dist[i][k], dist[k][j]) {
                    if dist[i][j].is_none() || dist[i][j].unwrap() > dik + dkj {
                        dist[i][j] = Some(dik + dkj);
                        next[i][j] = next[i][k];
                    }
                }
            }
        }
    }
    next
}

fn shortest_path(
    dist: &Vec<Vec<Option<usize>>>,
    next: &Vec<Vec<usize>>,
    start: usize,
    goal: usize,
) -> Option<(usize, Vec<usize>)> {
    if dist[start][goal].is_none() {
        return None;
    }

    let mut path = vec![start];
    let mut node = start;
    while node != goal {
        path.push(next[node][goal]);
        node = next[node][goal];
    }
    Some((dist[start][goal].unwrap(), path))
}

// fn modify(load: Vec<(i32, i32, i32)>) -> Vec<Vec<Option<i32>> {
//
// }

#[argio(output = AtCoder)]
fn main(n: usize, m: usize, t: [(usize, usize, usize); m]) {
    let (mut non_self, selfs) = check_self(t, n);
    let paths = warshall_floyd(&mut non_self);
    for i in 0..n {
        let w_short = shortest_path(&non_self, &paths, i, i);
        if let Some(e) = w_short {
            print!("{} ", if e.0 > selfs[i] { selfs[i] } else { e.0 });
        } else {
            if selfs[i] == usize::MAX {
                print!("-1 ")
            } else {
                print!("{} ", selfs[i])
            }
        }
    }
}
