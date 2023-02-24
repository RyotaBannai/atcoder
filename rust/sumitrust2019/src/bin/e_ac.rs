use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
use ac_library_rs::modint::ModInt1000000007 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Colorful Hats 2
 *
 * https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_e
 *
 * tags: #順列 #樹形図 #貪欲法
 *
 * 6
 * 0 0 1 0 1 2
 * のケースで考えるとわかりやすい
 * i=0 の時 xs[i]のケースは一つも出てきていなから、3色から選ぶ
 * i=1 の時 xs[i]のケースはi=0で１つだけ出てきているから2色から選ぶ
 * i=2 の時 xs[i] より１だけ少ない色は２つ考えられて、xs[i]のケースはまだ一つも出てきていないから2色から選ぶ
 * i=3 の時 xs[i]のケースはi=0,1で2つだけ出てきているから2色から選ぶ1色から選ぶ
 * i=4 の時 xs[i] より１だけ少ない色は２つ考えられて、xs[i]のケースはi=2で一つだけ出てきているから1色から選ぶ
 * i=5 の時 xs[i] より１だけ少ない色は２つ考えられて、xs[i]のケースはまだ一つも出てきていないから2色から選ぶ
 * ans = 3*2*(2-0)*1*(2-1)*2
 *
 * 以上を実装する.
 *
 * 色をa,b,c として樹形図を書くとよい
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [usize; n]
    }
    let mut ans = Mint::new(1usize);
    let mut count = vec![0; 1_000_005]; // i 番目まで確保?

    // x は x より前に出ている同じ色の個数 i.g. 0,1,4,3, など
    for x in xs {
        if x == 0 {
            ans *= 3 - count[x];
        } else {
            ans *= count[x - 1] - count[x];
        }
        count[x] += 1;
    }
    println!("{}", ans);
}
