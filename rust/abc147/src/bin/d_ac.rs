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
// type Map = BTreeMap<usize, Vec<usize>>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
/**
 * Xor Sum 4
 *
 * https://atcoder.jp/contests/abc147/tasks/abc147_d
 *
 * tags: #xor #sum #桁ごとに計算 #組み合わせ
 *
 *
 *
 */
// #[fastout]
#[allow(clippy::or_fun_call)]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut x = Mint::new(1usize);
    let mut ans = Mint::new(0usize);
    // 最大2^60 より小さいから、2^59 まで試す(2進数表記で59桁まで試す)
    for i in 0..60 {
        let mut one = 0;
        for &bit in a.iter() {
            if (bit & (1 << i)) != 0 {
                one += 1;
            }
        }
        let mut count = x;
        // i桁目における,0の数と1の数の積が組み合わせ数となる(0^1 の時i 桁目が1 となる組み合わせの数)
        count *= one;
        count *= n - one;
        ans += count;
        x *= 2;
    }
    println!("{}", ans);
}
