use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<(isize, isize), bool>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};
use std::isize::MAX;

#[fastout]
fn main() {
    input! {
        h: isize,
        w: isize,
        mut s: (isize,isize),
        mut e: (isize,isize),
        t: [Chars; h]
    }
    s = (s.0 - 1, s.1 - 1);
    e = (e.0 - 1, e.1 - 1);

    let mut q = VecDeque::new();
    q.push_back((s, -1, -1)); // start, direction change, prev direction :=初回で方向がない
    let mut mi = MAX;
    let mut v = vec![vec![MAX; 1005]; 1005];

    while let Some((u, count, prev_direc)) = q.pop_front() {
        if u == e {
            mi = mi.min(count);
            continue;
        }
        let (r, c) = u;
        if v[r as usize][c as usize] <= count || mi <= count {
            continue;
        }
        v[r as usize][c as usize] = count;

        // 上右下左　時計回り
        for (direc, (dr, dc)) in vec![(-1, 0), (0, 1), (1, 0), (0, -1)].iter().enumerate() {
            let (nr, nc) = (r + dr, c + dc);
            if 0 <= nr && nr < h && 0 <= nc && nc < w && t[nr as usize][nc as usize] == '.' {
                // 迷路の中にある
                let dcount = if prev_direc == direc as isize { 0 } else { 1 };
                q.push_back(((nr, nc), count + dcount, direc as isize));
            }
        }
    }
    println!("{}", mi);
}
