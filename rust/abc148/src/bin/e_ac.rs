use library::min;
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
 * Double Factorial
 *
 * https://atcoder.jp/contests/abc148/tasks/abc148_e
 *
 * tags: #math #素数 #階乗 #二重階乗
 *
 * 二重階乗: https://manabitimes.jp/math/1369
 *
 * - f は二重階乗を作る関数
 * - g: 階乗に含まれる素数p の数を求める関数. として、
 *   - g(n,p)= n/p + g(n/p,p) となるから、計算量は O(logN)
 * - 「末尾に0 が何個続くか」 <=> 「2*5 の組みが何個含まれるか」だから、2と5 の個数の最小
 * - n が偶数の時、素数2と5 が何個含まれるか？
 *   - a=n/2 で二重階乗に含まれる個数 + n! に含まれる2の個数を g(n/2, 2)
 *   - b=n! に含まれる5の個数を g(n/2, 5)
 *   - min(a,b)
 *
 * - n が奇数の時、素数2と5 が何個含まれるか？
 *   - 奇数の二重階乗 n!!について、n!!=n!/(n-1)!! でもとまる.
 *   - n! の素数2,5 の含まれる数から、偶数の二重階乗に含まれる素数2,5 の数を引くと、奇数の二重階乗n!! に含まれる2,5 の数がわかる.
 *   - 奇数の二重階乗には２が含まれないから、0 としても良い.
 *
 *
 * 参考
 * https://www.youtube.com/watch?v=F2p_e6iKxnk
 *
 */

fn g(x: usize, p: usize) -> usize {
    if x / p == 0 {
        return 0;
    }
    x / p + g(x / p, p)
}
// #[fastout]
fn main() {
    input! {
        n: usize
    }

    if n % 2 == 0 {
        // 偶数の場合
        let two = n / 2 + g(n / 2, 2);
        let five = g(n / 2, 5);
        println!("{}", min!(two, five));
    } else {
        // 奇数の場合
        // n!!=n!/(n-1)!!

        // let a = g(n, 2);
        // let b = g(n, 5);
        // let c = (n - 1) / 2 + g((n - 1) / 2, 2);
        // let d = g((n - 1) / 2, 5);
        // println!("{}", min!(a - c, b - d));

        println!("0");
    }
}
