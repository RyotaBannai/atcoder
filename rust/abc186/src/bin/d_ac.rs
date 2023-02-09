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
// use std::collections::{BinaryHeap, VecDeque};

/**
 *
 * Sum of difference
 *
 * https://atcoder.jp/contests/abc186/tasks/abc186_d
 *
 * tags: #式変形 #シグマの計算 #累積和
 *
 * 昇順にソートした時に i<j の組みで, Aj-Ai >=0 より絶対値が外れることを利用する
 * 内側のΣ はΣAj - (N-i)Ai に分解できるため、Σを累積和で求めておくことができる
 *
 * (N-i)Ai は1<=i<=N までで、最後のN 番目の時は組みはないから 0
 * ΣAj は、初回は A2~AN までの累積和で、i=N では0
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [isize; n]
    }
    a.sort_unstable();

    let mut ans = 0;
    let mut sum = 0;
    for (i, &x) in a.iter().enumerate() {
        sum += x;
        ans -= (n - i - 1) as isize * x;
    }

    for x in a {
        sum -= x;
        ans += sum;
    }

    println!("{}", ans);
}
