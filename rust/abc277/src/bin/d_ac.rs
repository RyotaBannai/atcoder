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
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Takahashi's Solitaire
 *
 * https://atcoder.jp/contests/abc277/tasks/abc277_d
 *
 * tags: #startから始める
 *
 * 参考：
 * https://www.youtube.com/watch?v=MihsrPsjA1c&ab_channel=%E3%81%8B%E3%81%A4%E3%81%A3%E3%81%B1%E7%AB%B6%E3%83%97%E3%83%AD
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize;n]
    }

    a.sort_unstable();
    // 条件を満たさない初めの開始位置を見つける
    let mut start = 0;
    for i in 0..n - 1 {
        if a[i + 1] - a[i] > 1 {
            start = i + 1;
            break;
        }
    }

    let sum = a.iter().sum::<usize>();
    // println!("{:?}", &a);
    // println!("sum {}", &sum);

    let mut tmp = 0;
    let mut mi = std::usize::MAX;
    for i in 0..n {
        let now = (i + start) % n;
        let prev = (now + n - 1) % n;
        if a[now] != a[prev] && a[now] % m != (a[prev] + 1) % m {
            mi = mi.min(sum - tmp);
            // 条件を満たさなくなる回に入ったから、リセットして後段の tmp で新しいレンジの初めの数字を加えるようにする
            tmp = 0;
        }
        // 都度必要に応じてリセットしたり和を管理しているから累積和等のテクニックは不要.
        tmp += a[now];
    }

    // 最後は条件を満たして完了するから、for から抜けた時に再チェックする
    mi = mi.min(sum - tmp);

    println!("{}", if mi == std::usize::MAX { 0 } else { mi });
}
