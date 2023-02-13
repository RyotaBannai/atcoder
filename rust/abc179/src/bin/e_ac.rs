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
 * Sequence Sum
 *
 * https://atcoder.jp/contests/abc179/tasks/abc179_e
 *
 * tags: #sum #鳩の巣の原理
 *
 * 解説
 * > Ai​ の取りうる値は M 通りしかありません。したがって、途中からは同じ数列を繰り返します。
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        m: usize
    }

    let f = |arg: &mut usize| {
        *arg *= *arg;
        *arg %= m;
    };

    let mut pos = vec![0; m + 1]; // 整数n が見つかった時に何回目か
    let mut val = vec![0; m + 1]; // i 回目の操作時のf の返却値
    let mut acc = x; // 周期が見つかるまでの和
    let mut a = x;
    let mut i = 1;
    loop {
        if pos[a] != 0 {
            // 周期が見つかった
            break;
        }
        if i >= n {
            // 周期が来るよりもはやく完了する
            println!("{}", acc);
            return;
        }
        pos[a] = i;
        val[i] = a;
        f(&mut a);
        acc += a;
        i += 1;
    }
    // loop から出る時のi は次の周期の１回目

    let mut sum = 0; // 周期内のsum
    let mut b = val[pos[a]];
    for _ in pos[a]..i {
        sum += b;
        f(&mut b);
    }
    let mut ans = acc - val[pos[a]];
    ans += ((n - i + 1) / (i - pos[a])) * sum;

    let rest = (n - i + 1) % (i - pos[a]);
    let mut c = a;
    for _ in 0..rest {
        ans += c;
        f(&mut c);
    }

    println!("{}", ans);
}
