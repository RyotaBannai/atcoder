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
// type Map = BTreeMap<usize, bool>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * https://atcoder.jp/contests/ddcc2020-qual/tasks/ddcc2020_qual_d
 *
 * tags: #整数の性質
 *
 * 基本的には、整数の桁数分ラウンドがあると考える. ただ、その際に、
 * 隣り合う二つの整数の和が 10 以上の時は桁数は変わらないから、その分ラウンド数が増える.
 * ゆえに
 * 「桁数-1 + 10 以上になって桁が増える回数」 と考える. -1 は最後の一桁で完了するため.
 * 後半の部分は、どの二つを足しても結局は全ての桁を足していくことから総和求めることと同じ. (ただし、総和が9 の倍数になる時に注意.)
 *
 * 注意：
 * ある整数で割る操作が入る時には、ちょうど割り切れた時の結果に問題がないか注意する
 *
 * 参考
 * https://drken1215.hatenablog.com/entry/2020/01/25/221900
 */

#[fastout]
fn main() {
    input! {
        m: usize,
        dc: [(usize, usize); m]
    }

    let mut sum = 0;
    let mut digit = 0usize;
    for (d, c) in dc {
        sum += d * c;
        digit += c;
    }

    // sum から -1 するのは、最後に 9 人まで残れるから 9/9 の計算結果は１余分にカウントしてしまうことになるため
    // 45 -> 9 なら (2桁-1) + 9/9 =  2 であるがこれは不適切な答え.
    // 2638 -> 838 -> 118 -> 19 -> 10 -> 1 なら (4桁-1) + 19/9 = 5 でokだけど、
    // 1638 -> 738 -> 108 -> 18 -> 9 なら、 (4桁-1) + 18/9 = 5 は不適切.
    println!("{}", (digit - 1) + (sum - 1) / 9);
}
