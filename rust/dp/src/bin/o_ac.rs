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
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * O - Matching
 *
 * https://atcoder.jp/contests/dp/tasks/dp_o
 *
 * tags: #dp #bit_dp
 *
 * 参考
 * https://www.youtube.com/watch?v=z27dePCa3Kc&t=140
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n]
    }

    let mo = 1000000007usize;
    let mut dp = vec![0usize; 1 << n]; // キーは組み合わせの状態 // 値は組み合わせ数
    let mut popcnt = vec![0; 1 << 21];
    dp[0] = 1;

    // 男性を前から一人づつ使った時、組になった女性の状態を遷移させる
    for bit in 1..1 << n {
        // popcnt := lady の集合の pop count 数 = 男性 i 番目まで使った、と考えられる
        popcnt[bit] = popcnt[bit / 2] + bit % 2;
        // 見ている lady の集合に対して、特定の lady l が含まれる時に、それを男性 popcnt[bit] - 1 との相性が良いかどうかをみる
        // popcnt[bit] - 1 は単に i 番目の男性を指して、bit から i 番目の男性を対象するた目に使っている
        for l in 0..n {
            if bit & 1usize << l == 0 {
                // この集合に lady l が含まれない
                continue;
            }
            // 男性 i との相性が良い時は、lady l を含まない集合を lady l も含めた集合として数え上げる
            dp[bit] += dp[bit - (1usize << l)] * a[popcnt[bit] - 1][l];
            dp[bit] %= mo;
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}
