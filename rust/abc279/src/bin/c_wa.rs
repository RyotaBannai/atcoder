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
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }

    // チェックせずに残っているカラムを管理.
    let mut cols = Set::new();
    for i in 0..w {
        cols.insert(i);
    }

    for j in 0..w {
        let mut rmv = -1;
        // cols: t の残っているカラム
        for &k in cols.iter().clone() {
            let mut flag = true;
            for i in 0..h {
                if s[i][j] != t[i][k] {
                    flag = false;
                    break;
                }
            }
            // 一致するカラムを見つけたら
            if flag {
                rmv = k as isize;
                break;
            }
        }
        if rmv != -1 {
            cols.remove(&(rmv as usize));
        }
    }

    if cols.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
