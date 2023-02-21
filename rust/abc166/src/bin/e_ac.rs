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
type Map = BTreeMap<isize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * This Message Will Self-Destruct in 5s
 *
 * https://atcoder.jp/contests/abc166/tasks/abc166_e
 *
 * 二つの組みを作りたい時には、複数変数があるときはそれらをまとめて、片方を固定できるようにする.(式変形など)
 *
 * tags: #math
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    // i<j として
    // j-i = ai+aj をindex でまとめる
    // j-aj = ai+i

    let mut ans = 0;
    let mut m = Map::new();
    for i in 0..n {
        let aj = (i + 1) as isize - a[i] as isize;
        let ai = (i + 1) as isize + a[i] as isize;
        if let Some(count) = m.get(&aj) {
            ans += count;
        }
        *m.entry(ai).or_insert(0) += 1;
    }
    println!("{}", ans);
}
