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
 * IPFL
 *
 * https://atcoder.jp/contests/abc199/tasks/abc199_c
 *
 * tags: #swap
 *
 * swap は直接操作しても O(1) だけど、前後反転するのは O(N) かかるから工夫する
 *
 * 前後反転した時の swap する位置は一意にに決まるから、
 * それを計算する操作を入れれば、都度前後反転する必要は無い
 *
 * swap で値が変わっていなければ
 * 前後反転した配列を再度前後反転すると元に戻る
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        ts: [(usize, (usize,usize)); q],
    }

    let mut flip = false;
    for (t, (mut a, mut b)) in ts {
        if t == 1 {
            if flip {
                if a > n {
                    a -= n;
                } else {
                    a += n;
                }
                if b > n {
                    b -= n;
                } else {
                    b += n;
                }
            }
            s.swap(a - 1, b - 1);
        } else {
            flip = !flip;
        }
    }

    if flip {
        s = [&s[n..], &s[..n]].concat();
    }

    for c in s.iter() {
        print!("{}", c);
    }

    println!();
}
