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
 * Repsept
 *
 * https://atcoder.jp/contests/abc174/tasks/abc174_c
 *
 * tags: #鳩の巣の原理
 *
 * K の倍数になる時の整数はどれか？
 * ある n Mod K が繰り返されるときに、0 Mod K になることはないから、割り切れない
 * Mod K ということはどんな整数N の N Mod K も N Mod K < K となるから、最大K 回この処理を繰り返せば答えがわかる.
 * (K 回繰り返しても答えが出ない場合は、ある n Mod K が２回以上出現している <=> 繰り返しが起きている. または n Mod K を管理してもいい)
 *
 */
// #[fastout]
fn main() {
    input! {
         k: usize
    }
    let mut prev = 7;
    for i in 0..k {
        if prev % k == 0 {
            println!("{}", i + 1);
            return;
        }
        prev %= k;
        prev *= 10;
        prev += 7;
    }
    println!("-1");
}
