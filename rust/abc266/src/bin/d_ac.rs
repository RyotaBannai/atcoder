use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeSet, HashMap};
type Map = HashMap<(usize, usize), usize>; // t, x

// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 *
 * D - Snuke Panic (1D)
 *
 * https://atcoder.jp/contests/abc266/tasks/abc266_d
 *
 * tags: #dp
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        v: [(usize, usize, usize); n]
    }

    let last = v[v.len() - 1].0;
    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; 5]; last + 1]; // 時刻t の時に座標x での最大value
    dp[0][0] = Some(0);

    // 時刻t の時に得点を得られるかどうかを簡単に調べられるようデータ整形
    let mut map = Map::new();
    for (t, x, a) in v {
        map.insert((t, x), a);
    }
    for i in 0..last {
        for j in 0..5 {
            // もし元の移動開始位置 j から移動できるなら
            if let Some(x) = dp[i][j] {
                // 選択は -1,0,+1 に動いた場合を考慮
                for dx in &[-1, 0, 1] {
                    let nx = dx + j as isize;
                    if nx < 0 || nx > 4 {
                        continue;
                    }
                    let nj = nx as usize;
                    if let Some(point) = map.get(&(i + 1, nj)) {
                        // 新しい位置にいる時にポイントをゲットできる
                        dp[i + 1][nj] = dp[i + 1][nj].max(Some(x + point));
                    } else {
                        dp[i + 1][nj] = dp[i + 1][nj].max(Some(x));
                    }
                }
            }
        }
    }

    // println!("{:?}", &dp);

    let mut ma = 0;
    for &x in &dp[last] {
        if let Some(y) = x {
            ma = ma.max(y);
        }
    }
    println!("{}", ma);
}
