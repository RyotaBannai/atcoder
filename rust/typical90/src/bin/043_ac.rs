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

/**
 * 043 - Maze Challenge with Lack of Sleep（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_aq
 *
 * tags: #bsf #01bfs #ダイクストラ #拡張ダイクストラ
 *
 */

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
    q.push_back((s, 0, 0)); // start, direction change, prev direction :=初回で方向がない
    let mut mi = MAX;
    let mut v = vec![vec![[MAX; 4]; 1005]; 1005]; // 3次元目に座標(r,c) に到達した時の方向を持つ

    while let Some((u, count, prev_direc)) = q.pop_front() {
        if u == e {
            mi = mi.min(count);
            continue;
        }

        let (r, c) = u;
        // 注意：count（ある座標(r,c) に到達するまでに方向を変えた回数）が同じでも前の方向が異なるから、必ずしもその時点で min/max の判断はできない
        if v[r as usize][c as usize][prev_direc] <= count || mi <= count {
            continue;
        }
        v[r as usize][c as usize][prev_direc] = count;

        // 上右下左　時計回り
        for (direc, (dr, dc)) in vec![(-1, 0), (0, 1), (1, 0), (0, -1)].iter().enumerate() {
            // 開始地点でない、かつ 前の向きと今回の向きが反対方向を向いていない（0^2 or 1^3）
            if s != u && (prev_direc ^ direc == 2) {
                continue;
            }
            let (nr, nc) = (r + dr, c + dc);
            if 0 <= nr && nr < h && 0 <= nc && nc < w && t[nr as usize][nc as usize] == '.' {
                // 迷路の中にある
                let dcount = if s == u || prev_direc == direc { 0 } else { 1 };
                q.push_back(((nr, nc), count + dcount, direc));
            }
        }
    }
    println!("{}", mi);
}
