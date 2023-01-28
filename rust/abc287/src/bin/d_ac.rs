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
 * Match or Not
 *
 * https://atcoder.jp/contests/abc287/tasks/abc287_d
 *
 * tags: #尺取り法
 *
 */
// #[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut n = t.len();
    let mut count = 0;
    for i in 0..n {
        let a = s[s.len() - (n - i)];
        let b = t[i];
        if a != b && a != '?' && b != '?' {
            count += 1;
        }
    }

    if count > 0 {
        println!("No");
    } else {
        println!("Yes")
    }

    // 開始地点をずらす
    for x in 0..n {
        // s からとる数値
        let a = s[s.len() - (n - x)];
        let b = t[x];
        // println!("とる {} {}", a, b);
        if a != b && a != '?' && b != '?' {
            count -= 1;
        }

        // s に入れる数値
        let c = s[x];
        // println!("入れる {} {}", c, b);
        if c != b && c != '?' && b != '?' {
            count += 1;
        }

        if count > 0 {
            println!("No");
        } else {
            println!("Yes")
        }
    }
}
