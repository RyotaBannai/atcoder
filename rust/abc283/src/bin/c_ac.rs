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
 * C - Cash Register
 *
 * https://atcoder.jp/contests/abc283/tasks/abc283_c
 *
 */
// #[fastout]
fn main() {
    input! {
        s: Chars
    }
    // 先頭から処理

    let n = s.len();

    let mut i = 0;
    let mut ans = 0;
    while i < n {
        if s[i] != '0' {
            ans += 1;
        } else {
            let mut cum_zero = 1;
            while i + 1 < n && s[i + 1] == '0' {
                i += 1;
                cum_zero += 1;
            }
            if cum_zero % 2 == 0 {
                ans += cum_zero / 2;
            } else {
                ans += (cum_zero / 2) + 1;
            }
            // while から出る時は、ちょうど次が 0 でない数値になっている.
        }
        i += 1;
    }

    println!("{}", ans);
}
