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
 * Road to Millionaire
 *
 * https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_d
 *
 * tags: #貪欲法
 *
 * i 日目に株を持っているかどうかで場合分.
 *  持っていて、明日の株の値段が今日以上であれば保持（何もしない）、そうでなければ売る
 *  持っていなくて、明日の株の値段が今日以上であれば買い、そうでなければ何もしない.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut sum = 1000;
    let mut count = 0;
    for i in 0..n - 1 {
        if count > 0 {
            // 株持ってる
            if a[i] <= a[i + 1] {
                // 明日値が上がるなら、保持.（何もしない）
                continue;
            } else {
                // 明日値が下がるなら、精算.
                // println!("{} {} {}", a[i], price, count);
                sum += a[i] * count;
                count = 0;
            }
        } else {
            // 持ってない
            if a[i] <= a[i + 1] {
                count = sum / a[i];
                sum -= count * a[i];
            }
        }
    }
    if count > 0 {
        sum += a[n - 1] * count;
    }
    println!("{}", sum);
}
