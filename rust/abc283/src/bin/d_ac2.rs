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
type Map = BTreeMap<char, bool>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Scope
 *
 * https://atcoder.jp/contests/abc283/tasks/abc283_d
 *
 * 最大の整数j は直近の'(' と考える.
 *
 */

// #[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut q = VecDeque::new();
    let mut bx = Map::new();
    for c in s {
        if c == '(' {
            q.push_back(c);
        } else if c == ')' {
            while let Some(x) = q.pop_back() {
                if x != '(' {
                    bx.remove(&x);
                } else {
                    // '(' が見つかったら箱からの取り出し完了
                    break;
                }
            }
        } else {
            if bx.contains_key(&c) {
                println!("No");
                return;
            }
            bx.insert(c, true);
            q.push_back(c);
        }
    }

    println!("Yes");
}
