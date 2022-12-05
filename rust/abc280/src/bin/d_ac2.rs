use num_integer::Roots;
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
use library::*;

/**
 * D - Factorial and Multiple
 *
 * https://atcoder.jp/contests/abc280/tasks/abc280_d
 *
 * tags: #素数 #素因数分解
 *
 */

// #[fastout]
fn main() {
    input! {
        mut l: usize
    }
    let mut ans = 1;
    let mut k = l;
    for i in (2..).take_while(|i| i * i <= l) {
        // 初めにある素数 p が k に何個含まれているか判定する. k=12, 2:2, 3:1
        let mut a = 0;
        while k % i == 0 {
            k /= i;
            a += 1;
        }
        // a の数を何番目の 整数n まで見れば削ることができるか調べる.
        // 2 を 2回削るには、4 までみる必要がある.
        // 他には 5 を2回削るには、5,10 で2回現れるから、10まで
        // もし、5 が6 個含まれているときは、5,10,15,20,25 まで見れば良い.
        // 少し複雑なことをしているのは、例えば「25 では 5 を２回削れる」のようなケースを想定していて、
        // 「5 の倍数で単に１回だけ削る」のではなく、25に含まれている5 の数分だけ削れるようにしている.
        let mut n = 0;
        while a > 0 {
            n += i;
            let mut x = n;
            while x % i == 0 {
                x /= i;
                a -= 1;
            }
        }
        chmax!(ans, n);
    }
    // k が10^6 より大きい素数の時、平方根以下のどの素数でも割れないから、n=0, therefore max(ans)=1 となる.
    chmax!(ans, k);

    println!("{}", ans);
}
