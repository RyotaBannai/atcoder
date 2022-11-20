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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use multiset::HashMultiSet;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * E - Grid Filling
 *
 * https://atcoder.jp/contests/abc278/tasks/abc278_e
 *
 * tags: #尺取法 #window
 *  
 * 塗りつぶしたいマスを１つづつずらして行くことで、O(N) 分削減する. (window をずらす)
 * 塗りつぶしたい部分以外にどの要素がどれくらいあるかを管理するのに mapや set（multiset）の insert, remove を使うと、
 * vec で管理する時に比べて、10 倍〜以上遅いからそれで TLE になる.
 * この場合N<=300 と小さいから、vec で持つ必要がある.
 *
 *
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        hh: usize,
        ww: usize,
        a: [[usize ;w]; h],
    }

    let mut cnt = vec![0isize; n + 1];

    for i in 0..h {
        for j in 0..w {
            cnt[a[i][j]] += 1;
        }
    }
    for i in 0..hh {
        for j in 0..ww {
            cnt[a[i][j]] -= 1;
        }
    }

    let mut v = vec![vec![vec![0isize; n + 1]; w - ww + 1]; h - hh + 1];
    v[0][0] = cnt;

    // i, j を左上のマスとする
    for i in 0..=h - hh {
        if i >= 1 {
            // 二段目以降は行方向と同じ処理を施す
            let mut cnt = v[i - 1][0].clone(); // 一つスライドするから、一つ上の結果を取り出す
            for c in 0..ww {
                cnt[a[i - 1][c]] += 1;
                cnt[a[i + hh - 1][c]] -= 1;
            }
            v[i][0] = cnt;
        }

        // j=0 は i>=1 の処理で済んでいるから j=1 から考える
        for j in 1..=w - ww {
            let mut cnt = v[i][j - 1].clone(); // 一つスライドするから、一つ左の結果を取り出す.
            for r in 0..hh {
                cnt[a[i + r][j - 1]] += 1;
                cnt[a[i + r][j + ww - 1]] -= 1;
            }
            v[i][j] = cnt;
        }
    }

    for i in 0..=h - hh {
        for j in 0..=w - ww {
            print!("{} ", v[i][j].iter().filter(|&&x| x != 0).count());
        }
        println!();
    }
}
