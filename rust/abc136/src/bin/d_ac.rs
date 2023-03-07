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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Gathering Children
 *
 * https://atcoder.jp/contests/abc136/tasks/abc136_d
 *
 * tags: #貪欲法
 *
 * とても大きい偶数回後にはRL のループに収束する
 *
 * L 時には、range(..=i).last() のように最も後方にある位置を見る(last を使う)
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        s: Chars
    }
    let n = s.len();

    // マスがR で右がL なら大きい偶数回後に同じ場所に戻る
    // マスがR で右もR なら到達したL の右側のRかLに収束 RRRL
    // マスがL で左がR なら大きい偶数回後に同じ場所に戻る

    let mut l = Set::new(); // rl と連続するマスのl の位置
    for i in 0..n - 1 {
        if s[i] == 'R' && s[i + 1] == 'L' {
            l.insert(i + 1);
        }
    }
    let mut count = vec![0; n];
    for i in 0..n {
        if s[i] == 'R' {
            if let Some(&j) = l.range(i..).next() {
                if (j - i) % 2 == 0 {
                    // 偶数距離
                    count[j] += 1;
                } else {
                    // 奇数距離
                    count[j - 1] += 1;
                }
            }
        }

        if s[i] == 'L' {
            if let Some(&j) = l.range(..=i).last() {
                if (i - j) % 2 == 0 {
                    // 偶数距離
                    count[j] += 1;
                } else {
                    // 奇数距離
                    count[j - 1] += 1;
                }
            }
        }
    }

    for x in count {
        print!("{} ", x);
    }
    println!();
}
