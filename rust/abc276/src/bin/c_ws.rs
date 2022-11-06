use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * @workspace
 *
 * C - Previous Permutation
 *
 * https://atcoder.jp/contests/abc276/tasks/abc276_c
 *
 * tags: #next_permutation #prev_permutation
 *
 */

#[allow(dead_code)]
fn next_permutation(mut p: Vec<usize>) -> Vec<usize> {
    let n = p.len();
    let mut set = Set::new(); // 数値, index
    let mut pos = 0;
    for i in (1..n).rev() {
        set.insert((p[i], i));
        if p[i - 1] < p[i] {
            pos = i;
            break;
        }
    }

    let a = p[pos - 1];
    let mut v = p[pos..].to_vec();
    v.sort_unstable();
    let np = v.upper_bound(&a);
    let b = v[np];
    let orig_pos = set.range((b, 0)..).next().unwrap().1;

    p.swap(pos - 1, orig_pos);
    p[pos..].reverse();
    p
}

#[allow(dead_code)]
fn prev_permutation(mut p: Vec<usize>) -> Vec<usize> {
    let n = p.len();
    let mut set = Set::new(); // 数値, index
    let mut pos = 0;
    for i in (1..n).rev() {
        set.insert((p[i], i));
        if p[i - 1] > p[i] {
            // １の桁から見て、大きい桁の数値の方がより大きい場合に探索終了
            pos = i; // 単調増加していた位置の一番前のindex
            break;
        }
    }

    let a = p[pos - 1];
    // 単調増加が完了した位置の数値より一つ小さい数値を探したい
    let mut v = p[pos..].to_vec();
    v.sort_unstable();
    let mut np = v.lower_bound(&a);
    np -= 1; // 一つ小さい位置
    let b = v[np]; // 一つ小さい数値を見つけた
    let orig_pos = set.range((b, 0)..).next().unwrap().1;

    p.swap(pos - 1, orig_pos);
    p[pos..].reverse();
    p
}

#[fastout]
fn main() {
    println!();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut p = vec![1, 2, 5, 4, 3];
        let len = p.len();

        let a = next_permutation(p.clone());
        p.next_permutation();

        for i in 0..len {
            assert_eq!(a[i], p[i]);
        }
    }

    #[test]
    fn test_prev_permutation() {
        let mut p = vec![2, 1, 3, 4, 5];
        let len = p.len();

        let a = prev_permutation(p.clone());
        p.prev_permutation();

        for i in 0..len {
            assert_eq!(a[i], p[i]);
        }
    }
}
