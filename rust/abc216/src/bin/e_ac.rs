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
 * E - Amusement Park
 *
 * https://atcoder.jp/contests/abc216/tasks/abc216_e
 *
 * tags: #等差数列の和
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        mut k: isize,
        mut a: [isize; n]
    }

    a.push(0);
    a.sort_unstable();
    let mut sub = vec![0; n + 1];
    for i in 0..n {
        sub[i + 1] = a[i + 1] - a[i];
    }
    // println!("{:?}", &sub);
    let mut count = 1;
    let mut sum = 0;
    for (i, &x) in sub.iter().enumerate().rev() {
        if count * x > k {
            // 取り出せる数が少ない時
            let p = k / count;
            let rest = k - count * p;

            sum += p * (a[i] + (a[i] - p + 1)) / 2 * count;

            sum += (a[i] - p) * rest;

            break;
        } else {
            // この回は全て取り出せる
            sum += x * (a[i] + (a[i] - x + 1)) / 2 * count;
            k -= x * count; // 使える数分 * それまでの累積個数を掛ける
            count += 1;
            if k == 0 {
                break;
            }
        }
    }
    println!("{}", sum);
}
