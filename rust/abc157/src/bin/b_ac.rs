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
 * Bingo
 *
 * https://atcoder.jp/contests/abc157/tasks/abc157_b
 */
// #[fastout]
fn main() {
    input! {
        mut g: [[usize; 3]; 3],
        n: usize,
        b: [usize; n]
    }
    for x in b {
        for i in 0..3 {
            for j in 0..3 {
                if g[i][j] == x {
                    g[i][j] = 0;
                }
            }
        }
    }
    // 行
    for i in 0..3 {
        let mut ok = true;
        for j in 0..3 {
            if g[i][j] != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }

    // 列
    for j in 0..3 {
        let mut ok = true;
        for i in 0..3 {
            if g[i][j] != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }

    // 斜め
    let mut ok = true;
    for k in 0..3 {
        if g[k][k] != 0 {
            ok = false;
            break;
        }
    }
    if ok {
        println!("Yes");
        return;
    }
    let mut ok = true;
    for k in 0..3 {
        if g[2 - k][k] != 0 {
            ok = false;
            break;
        }
    }
    if ok {
        println!("Yes");
        return;
    }

    println!("No");
}
