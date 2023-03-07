use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt1000000007 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<(usize, usize), Mint>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::number::combination::*;

/**
 * Knight
 *
 * https://atcoder.jp/contests/abc145/tasks/abc145_d
 *
 * tags: #組合せ #パスカルの三角形 #二項定理 #math
 *
 * 繊維が２通りしかない場合は、二項定理を考える.
 *
 * コマの移動できるマスを数え上げるとパスカルの三角形に一致
 * n 段目で左からk 番目の組み合わせは n-1Ck-1 で求められる.
 * n 段目かどうかは、コマは一回の移動で(1,2) または(2,1) だけ移動するから、(y,x)から(x+y)/3 でもとまる.
 *
 * さらに n回分（1,2）を移動したときには、y=n となるから、(n,2*n) がn 段目の左端になることがわかる.
 * ゆえに(y,x) は左から y-n+1 番目=k
 *
 * この条件を満たさない場合は、(y,x)に到達できないケースだから、0 を返す.
 *
 *
 *
 *
 */

// #[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    }
    let n = (x + y) / 3; // 何回目で到達できるか
    if (x + y) % 3 != 0 || y < n || x > 2 * n {
        println!("0");
        return;
    }
    let k = y - n;
    if k > n {
        println!("0");
        return;
    }
    println!("{}", combination(n, k, 1000000007));
}
