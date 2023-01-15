use proconio::{fastout, input, marker::Chars};
use std::cmp::{
    max, min,
    Ordering::{Equal, Greater, Less},
};

// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<char, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Neo-lexicographic Ordering
 *
 * https://atcoder.jp/contests/abc219/tasks/abc219_c
 *
 * tags: #辞書順
 *
 * 辞書順とは？
 *
 * 辞書順とは簡単に説明すると「単語が辞書に載っている順番」を意味します。より厳密な説明として、英小文字からなる相異なる文字列
 * S,T の大小を判定するアルゴリズムを以下に説明します。
 * 以下では「S の i 文字目の文字」を Si​ のように表します。また、 S が T より辞書順で小さい場合は S<T 、大きい場合はS>T と表します。
 * S,T のうち長さが大きくない方の文字列の長さを L とします。i=1,2,…,L に対して Si​ と
 * Ti​ が一致するか調べます。
 * Si=Ti​ である i が存在する場合、そのような i のうち最小のものを j とします。そして、Sj​ と Tj​ を比較して、Sj​ が Tj​ よりアルファベット順で小さ
 * 場合は S<T 、そうでない場合はS>T と決定して、アルゴリズムを終了します。
 * Si=Ti​ である i が存在しない場合、S と T の長さを比較して、S が T より短い場合は S<T 、長い場合は S>T と決定して、アルゴリズムを終了します。
 *
 *
 */
use library::utils::conv::alp_to_i;

// #[fastout]
fn main() {
    input! {
        x: Chars,
        n: usize,
        mut s: [Chars; n]
    }

    let mut m = vec![0; 26];
    for (i, x) in x.iter().enumerate() {
        m[alp_to_i(*x)] = i;
    }
    let cmp = |a: &Vec<char>, b: &Vec<char>| {
        for (&i, &j) in a.iter().zip(&mut b.iter()) {
            let ret = m[alp_to_i(i)].cmp(&m[alp_to_i(j)]);
            if ret != Equal {
                return ret;
            }
        }
        // 短い方の文字列比較した後に等しければ、
        // 文字列の長さで比較する.
        a.len().cmp(&b.len())
    };
    s.sort_unstable_by(cmp);

    for xs in s {
        for x in xs {
            print!("{}", x);
        }
        println!();
    }
}
