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
 * Takahashi Unevolved
 *
 * https://atcoder.jp/contests/abc180/tasks/abc180_d
 *
 * tags: #貪欲法
 *
 * a>=2 が保証されているから、結局は *a が +b より小さいうちだけ前者を採用して、
 * i回目の *a>+b となったら打ち切って、残りを +b だけで経験値を増やすのが良い
 *
 * この探索は2^i で完了するから log(N)
 *
 * 残りを b にした時にちょうどY になってしまってはいけないから、
 * 剰余==0 の時は一回分引く必要がある.
 *
 */
fn main() {
    input! {
        mut x: usize,
        y: usize,
        a: usize,
        b: usize,
    }

    // a >=2 より指数関数で logN
    let mut m = 0; // 経験値

    // x <=10^18 でusize としてはMAX に近いから、x*a がoverflow しないよう注意
    while x.saturating_mul(a) < x + b && x + b < y {
        x *= a;
        m += 1;
    }
    m += (y - x) / b;
    if (y - x) % b == 0 {
        m -= 1;
    }
    println!("{}", m);
}
