use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
use std::ops::Bound::Included;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Smallest Subsequence（★5）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_f
 *
 * tags: #辞書順 #辞書順最小 #貪欲法
 *
 * 参考
 * https://drken1215.hatenablog.com/entry/2021/10/10/195200
 * https://twitter.com/e869120/status/1379202843622576130/photo/1
 *
 *
 * 類題：
 * - ABC076-C 「Dubious Document 2」
 * - ABC007-D 「辞書式順序ふたたび」
 * JOI2021-春合宿 「Event Hoppoing 2」
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }

    let mut pos = vec![vec![None; n]; 26]; // 26: アルファベット数
    for (i, c) in s.iter().rev().enumerate() {
        let key = (*c as u8 - b'a') as usize;
        let last = n - i - 1;
        pos[key][last] = Some(last);
        for b in b'a'..=b'z' {
            let m = (b - b'a') as usize;
            if m != key && last + 1 != n && pos[m][last + 1] != None {
                pos[m][last] = pos[m][last + 1];
            }
        }
    }
    // debug
    // for (i, xs) in pos.iter().enumerate() {
    // println!("{} >> {:?}", (b'a' + i as u8) as char, &xs);
    // }
    let mut v = vec![];
    let mut prev_pos = -1isize;
    // k 文字取り出したい.
    for i in 0..k {
        // a から取り出すことが辞書順で早いため
        for b in b'a'..=b'z' {
            // println!("{}", b as char); // debug
            let m = (b - b'a') as usize;
            // 前から順位取り出す
            let j = (prev_pos + 1) as usize;
            if let Some(next_pos) = pos[m][j] {
                // debug
                // println!(">> {} >= {}", n - (next_pos + 1), k - (i + 1));

                // atcoder なら 3つ目のd を選んだ時は
                // '残り選びたい個数'k - (i + 1) == 0 で、d の'後ろにある個数'n - (next_pos + 1)==2 だからd を選ぶことができる.
                // 辞書順では a から見るのが常に前にくる並び方だから、選べる時点で最小の辞書順になる
                // 文字列の順序は考慮しないといけないから、次に選べる位置を現在選んだ位置のひとつ次に移動させる
                if n - (next_pos + 1) >= k - (i + 1) {
                    v.push(next_pos);
                    prev_pos = next_pos as isize;
                    break;
                }
            }
        }
    }

    for i in v {
        print!("{}", s[i]);
    }
    println!();
}
