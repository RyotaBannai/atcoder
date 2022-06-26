use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<char, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 008 - AtCounter（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_h
 *
 * tags: #map #map_dp
 *
 * 長さ n の文字列から 0 個以上の文字列を取り出して、順番を変えずに "atcoder" になるような
 * 取り出し方はどれくらいあるか？
 *
 * 文字が異なる index であれば区別される
 *
 * 例
 * attcordeer
 *
 * t が 2つ、e が 2つあるため、4 通り
 *
 * 前から探索？
 * attacordtcordeer
 * でも拾うには
 *
 * DP？
 * "atcoder" は文字が全てユニーク
 * 対象の文字の前にくる文字がそれが出現する前に何回出現しているか DP で管理？
 *
 * e が来たときの d ？ その時点での個数の map でいい
 *
 *
 * dp で解くには？ 基本的に　map_dp と同じ
 * ・"atcoder" の文字列分だけの vec![] を用意
 * ・loop で char の位置を "atcoder".find
 * ・自分の char のそれまでの組み合わせ + その前に来るべき文字の組み合わせ数で自分の char の vec を更新
 * ・最後に vec の 'r' を表示　done
 */

fn pre_c(c: char) -> char {
    match c {
        't' => 'a',
        'c' => 't',
        'o' => 'c',
        'd' => 'o',
        'e' => 'd',
        'r' => 'e',
        _ => 'z',
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: Chars
    }
    let mo = 1_000_000_007_usize;
    let ac = "atcoder";
    let mut m = Map::new();

    for c in ac.chars() {
        m.insert(c, 0);
    }

    for c in s {
        // 先頭なので個数を増やすだけ
        if c == 'a' {
            if let Some(v) = m.get_mut(&'a') {
                *v += 1;
                *v %= mo;
            }
        } else {
            let pre = pre_c(c); // ac[ac.find(&c)-1] でも良い
            if pre == 'z' {
                continue;
            }
            if let Some(&pre_v) = m.get(&pre) {
                if let Some(v) = m.get_mut(&c) {
                    *v += pre_v; // それまでの出現回数分パターンが増える
                    *v %= mo;
                }
            }
        }
    }

    println!("{}", m.get(&'r').unwrap());
}
