use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
// max, min,
// Ordering::{Equal, Greater, Less},
// };
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::min;

/**
 * Walking Takahashi
 *
 *
 * 原点0 に限りなく近づけた時の残りのステップ数を考えたときに、
 * 残りが奇数なら、原点の先の正負が反転する位置との最小を考える.
 * 残りが偶数なら、限りなく近づけた位置が最小になる.(残りの回数で今の位置に戻ってくることしかできない)
 *
 */

#[allow(clippy::collapsible_else_if)]
fn main() {
    input! {
        x: isize,
        k: isize,
        d: isize
    }

    if x < 0 {
        let p = (x / d).abs();
        if p >= k {
            // k回しか移動できない
            println!("{}", (x + d * k).abs());
        } else {
            if (k - p) % 2 == 0 {
                // 偶数回残っていれば0を跨いでもう一回戻ってきた時が最適
                println!("{}", (x + d * p).abs());
            } else {
                // 0を跨ぐのが最適か否かをチェック
                println!("{}", min!((x + d * p + d).abs(), (x + d * p - d).abs()));
            }
        }
    } else {
        let p = x / d;
        if p >= k {
            // k回しか移動できない
            println!("{}", x - d * k);
        } else {
            if (k - p) % 2 == 0 {
                // 偶数回残っていれば0を跨いでもう一回戻ってきた時が最適
                println!("{}", x - d * p);
            } else {
                // 0を跨ぐのが最適か否かをチェック
                println!("{}", min!((x - d * p - d).abs(), (x - d * p + d).abs()));
            }
        }
    }
}
