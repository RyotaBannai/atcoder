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
 * C - Previous Permutation
 *
 * https://atcoder.jp/contests/abc276/tasks/abc276_c
 *
 * tags: #prev_permutation
 *
 * 整数のある区間で桁が小さい方から、単調増加している範囲ではそれ以上小さくできないことがわかる.
 * 51234 この時上の内容は、1234 にあたる.
 * 51234 より小さい整数を求めたい時は、５より小さい、かつ1234 の中で一番大きい数値と5 を入れ替える必要があり、入れ替えた後は単調増加していた部分は単調減少することがわかる.
 *
 * 41235（入れ替え）-> 45321（単調減少） が次の値
 *
 * 必ずしも、単調増加部分の末尾が単調増加が終わった位置より大きいとは限らない.
 * 例えば、
 * 41235, 21345 など. 21345 では単調増加が 21 間で完了しているから、1 2 を入れ替え、単調減少にする. 12543
 *
 * 上記のことはどの桁でも成り立つことに注意.
 * 12354 なら、4 5 間で単調増加が完了しているから、'5 より小さく一番大きい数値' 4 と入れ替えて 12345 となる.
 *
 */

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
    p[pos - 1] = b;
    p[orig_pos] = a;
    p[pos..].reverse();
    p
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut p: [usize; n] //文字列で受け取る
    }

    for x in prev_permutation(p) {
        print!("{} ", x);
    }
    println!();
}
