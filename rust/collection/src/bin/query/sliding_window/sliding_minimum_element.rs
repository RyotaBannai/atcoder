/**
 * @cpg_dirspec sliding_minimum_element
 *
 * cpg run -p src/bin/query/other/sliding_window/sliding_minimum_element.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeSet, HashMap};
type Map = HashMap<usize, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::utils::read::*;

/**
 * Sliding Minimum Element
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/3/DSL_3_D
 *
 * tags: #区間最小
 *
 * set を使うと良い.  
 *
 */

// #[fastout]
fn main() {
    let x = read::<usize>();
    let (n, l) = (x[0], x[1]);
    let a = read::<usize>();

    let mut ans = vec![];
    let mut s = Set::new();
    let mut m = Map::new();
    for i in 0..l {
        *m.entry(a[i]).or_insert(0) += 1; // 要素の個数管理
        s.insert(a[i]); // 最小値を管理
    }
    if let Some(x) = s.range(0..).next() {
        ans.push(*x);
    }

    for r in l..n {
        // 追加
        *m.entry(a[r]).or_insert(0) += 1;
        s.insert(a[r]);
        // 削除 r==3 で l==2 の時、index=1(r-l) は削除したい
        let k = a[r - l];
        if let Some(x) = m.get_mut(&k) {
            *x -= 1;
            if *x == 0 {
                m.remove(&k);
                // 個数が0 になるなら、最小値も更新したいから、Set から削除
                s.remove(&k);
            }
        }
        if let Some(x) = s.range(0..).next() {
            ans.push(*x);
        }
    }

    for (i, &x) in ans.iter().enumerate() {
        print!("{}", x);
        if i != n - l {
            print!(" ");
        }
    }
    println!();
}
