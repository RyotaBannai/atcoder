use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<char, BTreeMap<usize, usize>>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 008alpha - AtCounter - 複数同じ文字が含まれる場合
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_h
 *
 * tags: #map #map_dp #multi_chars
 *
 * 長さ n の文字列から 0 個以上の文字列を取り出して、順番を変えずに長さ m の文字列になるような
 * 取り出し方はどれくらいあるか？
 *
 * 文字が異なる index であれば区別される
 *
 * 例
 * n l
 * s
 * t
 *
 * 7 8
 * okayama
 * okayyama
 *
 * 2 パターンが考えれる
 *
 *
 * 解き方
 * ・複数文字が出現する → 複数箇所で部分文字列の始まりになる → その出現箇所に対して、新しい文字を加える
 * ・複数箇所をカウントする場合は、一番後ろの文字から行う
 * 例
 * 'too' を見つけるとし、o が出現した時に o を前から加算すると、初期状態が {1:0, 2:0} -> {1:1, 2:2} になってしまう
 * ・同じ文字の位置とカウントをセットにした map で管理. -> map の map
 * ・map から各 index を見て、そのひとつ前の char の index のカウントを探す
 *
 *
 * okayyama ma 2 orig
 *
 * okayyamam a 2 with last a
 *
 * okayyama ma 4 yyaa
 *
 * ttl 8
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        mut s: Chars,
        mut t: Chars,
    }
    let mo = 1_000_000_007_usize;
    let mut m = Map::new();

    for (i, c) in t.iter().enumerate() {
        if let Some(v) = m.get_mut(c) {
            v.insert(i, 0);
        } else {
            let mut tmp = BTreeMap::new();
            tmp.insert(i, 0);
            m.insert(*c, tmp);
        }
    }

    let mut ref_m = m.to_owned();

    for c in &s {
        if let Some(c_pos_m) = m.get_mut(c) {
            // 後ろから更新
            for (pos, v) in c_pos_m.iter_mut().rev() {
                // 先頭なら無条件に加える
                if pos == &0 {
                    *v += 1;
                    continue;
                } else {
                    let pre_c = &t[pos - 1]; // 一つ前
                    if let Some(pre_c_pos_m) = ref_m.get(pre_c) {
                        let pre_c_cnt = pre_c_pos_m.get(&(pos - 1)).unwrap();
                        *v += pre_c_cnt;
                        *v %= mo;
                    }
                }
            }

            *ref_m.get_mut(c).unwrap() = c_pos_m.to_owned(); // 参照用の map を更新した文字の map 部分だけ更新
        }
    }

    let mm = m.get(t.last().unwrap()).unwrap();
    println!("{}", mm.get(&(l - 1)).unwrap());
}
