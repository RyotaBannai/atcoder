use itertools::Itertools;
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
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Hachi
 *
 * https://atcoder.jp/contests/abc181/tasks/abc181_d
 *
 * tags: #map
 *
 * 注意:
 * |s| = 2*10^5 の長さをもつから、usize でも管理できない-> 文字列として受け入れる
 *
 */

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut map = Map::new();
    for c in s {
        let k = c.to_digit(10).unwrap() as usize;
        if let Some(x) = map.get_mut(&k) {
            *x = (*x + 1).min(3); // 最大3つまで管理できれば下３桁見れる.
        } else {
            map.insert(k, 1);
        }
    }

    // (3*9)! だけなら間に合う.
    let mut v = vec![];
    for (x, c) in map {
        v.append(&mut vec![x; c]);
    }

    // println!("{:?}", &v);

    // 1o2桁の整数は下三桁みる必要はない
    if v.len() == 1 {
        if v[0] % 8 == 0 {
            println!("Yes");
            return;
        }
    } else if v.len() == 2 {
        if (v[0] * 10 + v[1]) % 8 == 0 || (v[1] * 10 + v[0]) % 8 == 0 {
            println!("Yes");
            return;
        }
    } else {
        // len=<3 の時は permutations の loop に入らない.（「組合せを作れない」）
        for y in v.iter().permutations(3) {
            let mut n = 0;
            let mut d = 1;
            for z in y {
                n += z * d;
                d *= 10;
            }
            if n % 8 == 0 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
