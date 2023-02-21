use library::number::gcd;
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
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Marking
 *
 * https://atcoder.jp/contests/abc290/tasks/abc290_d
 *
 * tags: #gcd #math
 *
 * n*d 回愚直にn+1 をせずに進んだ時の位置を L とすると、
 * L の位置を２回目に踏む時に必要なステップ数は何回だろうか？
 * これは n とd のgcd を求めて、n/gcd をすることで求めることができる.
 *
 * N=12, D=9 なら、gcd=3 であるから、L の位置に戻ってくるのに必要な回数は a=N/gcd=12/3=4 回
 * つまり4 回繰り返すと、L に戻ってきてn+1 に進まないといけないということだから、k/4 回分n+1 した位置
 * を求めれば良いことがわかる.
 *
 * k/a の結果 modN されてしまうのではないか？
 * n+1 した結果modN されて0 地点に戻ってくるケースはK<=A の制約があるから考えない
 *
 *
 * AとB をGCD を求めるとaA≡0(ModB) or bB≡0(ModA) になるようなa or b がわかる.(a!=0, b!=0)
 *
*/
// #[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: isize,
            d: isize,
            mut k: isize
        }
        k -= 1;
        let g = gcd(vec![n, d]);
        let a = n / g;
        println!("{}", k * d % n + k / a);
    }
}
