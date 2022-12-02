use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeSet, HashMap};
type Map = HashMap<usize, Vec<usize>>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Draw Your Cards
 *
 * https://atcoder.jp/contests/abc260/tasks/abc260_d
 *
 * テーブル上にあるカードを管理するDS には、重なったカードの枚数がk になったときに「削除する処理が高速な」 DS を選ぶ.
 * vec の remove は遅いため、set の remove を使う.
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize;n]
    }
    let mut m = Map::new();
    let mut s = Set::new();
    let mut t = vec![0; n + 1];
    for (i, &x) in p.iter().enumerate() {
        if let Some(&prev) = s.range(x..).next() {
            // x 以上が見つかった.
            let mut xs = m.remove(&prev).unwrap();
            xs.push(x);
            s.remove(&prev);
            if xs.len() == k {
                for y in xs {
                    t[y] = i + 1; // 食べる
                }
            } else {
                m.insert(x, xs);
                s.insert(x);
            }
        } else {
            // x 以上の数値が見つからなかった（全部自分より小さい）から、テーブルに表におく
            if k == 1 {
                t[x] = i + 1; // 食べる
            } else {
                m.insert(x, vec![x]);
                s.insert(x);
            }
        };
    }

    for &x in t.iter().skip(1) {
        if x == 0 {
            println!("-1");
        } else {
            println!("{}", x);
        }
    }
}
