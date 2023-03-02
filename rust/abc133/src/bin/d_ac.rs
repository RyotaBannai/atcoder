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
 * Rain Flows into Dams
 *
 * https://atcoder.jp/contests/abc133/tasks/abc133_d
 *
 * tags: #貪欲法
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }
    let mut total: usize = p.iter().cloned().sum();
    total /= 2;
    let mut ans = vec![0; n];
    {
        let mut sum = 0;
        for i in (1..n).step_by(2) {
            sum += p[i];
        }
        ans[0] = total - sum;
        for i in (2..n).step_by(2) {
            sum -= p[i - 1];
            sum += p[i - 2];
            ans[i] = total - sum;
        }
    }
    {
        let mut sum = 0;
        for i in (2..n).step_by(2) {
            sum += p[i];
        }
        ans[1] = total - sum;
        for i in (3..n).step_by(2) {
            sum -= p[i - 1];
            sum += p[i - 2];
            ans[i] = total - sum;
        }
    }
    for x in ans {
        print!("{} ", x * 2)
    }
}
