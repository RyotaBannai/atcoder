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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Cylinder
 *
 * https://atcoder.jp/contests/abc247/tasks/abc247_d
 *
 *
 * 取り出すときの操作が run length を使っても時間がかかりそうと思うが、
 * 入力の量はQ<=10^5 で 1 1 1 のような細かく追加された場合でも、取り出しの最大回数は高々 10^5.
 *
 * 仮に Q<=10^12 だとしても入力時点でTLE になるため問題として成立しない.
 */

// #[fastout]
#[allow(clippy::comparison_chain)]
fn main() {
    input! {
        q: usize,
    }
    let mut v = VecDeque::new();

    for _ in 0..q {
        input! {
            n: usize,
        }
        if n == 1 {
            input! {
                x: usize,
                c: usize,
            }
            v.push_front((x, c));
        } else {
            input! {
                mut c: usize,
            }
            let mut sum = 0;
            while let Some((x, count)) = v.pop_back() {
                if count < c {
                    sum += count * x;
                    c -= count; // 取り出した分減らす
                } else if count == c {
                    // ちょうど個数を取り出せる.
                    sum += count * x;
                    break;
                } else {
                    // count の方が取り出したい数より多い
                    sum += c * x;
                    v.push_back((x, count - c));
                    break;
                }
            }
            println!("{}", sum);
        }
    }
}
