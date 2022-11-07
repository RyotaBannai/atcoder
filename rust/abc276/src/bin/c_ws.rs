use itertools::Itertools;
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

// n の階乗 n!
#[allow(dead_code)]
fn fact(n: usize) -> Vec<usize> {
    let mut fact = vec![1; n + 1];
    for i in 0..n {
        fact[i + 1] = fact[i] * (i + 1);
    }
    fact
}

#[allow(dead_code)]
fn id_of_permutation(n: Vec<usize>) -> usize {
    let fact = fact(n.len());

    let mut set = BTreeSet::<usize>::new();
    for &x in &n {
        set.insert(x);
    }

    let mut k = 0;
    for &x in &n {
        set.remove(&x);
        let len = set.len();
        let m = set.range(..x).count(); // x より先にくる順列数
        k += m * fact[len];
    }

    k
}

// https://atcoder.jp/contests/abc276/editorial/5189
// https://atcoder.jp/contests/abc276/submissions/36265971
#[allow(dead_code)]
fn kth_permutation(n: usize, mut k: usize) -> Vec<usize> {
    let fact = fact(n);
    let mut v = vec![];
    let mut s = (0..n).collect_vec();
    for i in 0..n {
        let a = fact[n - 1 - i];
        let j = k / a;
        k %= a;
        v.push(s[j]);
        s = [&s[..j], &s[j + 1..]].concat();
    }

    v.iter().map(|x| x + 1).collect_vec()
}

#[fastout]
fn main() {
    todo!();

    // judge pass しない
    // input! { n: usize, mut p: [usize; n] }
    // let kth = id_of_permutation(p.clone());
    // let ret = kth_permutation(p.len(), kth - 1);

    // for x in ret {
    //     print!("{} ", x);
    // }
}

// https://stackoverflow.com/questions/40154150/how-do-i-concatenate-two-slices-in-rust

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use rstest::*;

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

    // https://crates.io/crates/rstest
    #[rstest]
    #[case(vec![3, 1, 2], 4, vec![2, 3, 1])]
    #[case(vec![1, 3, 2], 1, vec![1, 2, 3])]
    #[case(vec![4, 2, 1, 3], 20, vec![4, 1, 3, 2])]
    #[should_panic(expected = "attempt to multiply with overflow")]
    #[case((1..=40).collect_vec(), 20, vec![])]
    fn test_kth_permutation(
        #[case] input: Vec<usize>,
        #[case] expected_kth: usize,
        #[case] expected_output: Vec<usize>,
    ) {
        let len = input.len();
        let kth = id_of_permutation(input);
        assert_eq!(kth, expected_kth);

        let ret = kth_permutation(len, kth - 1);

        for i in 0..len {
            assert_eq!(ret[i], expected_output[i]);
        }
    }
}
