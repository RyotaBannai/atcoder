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
 * Snowy Aobayama
 *
 * https://atcoder.jp/contests/tupc2022/tasks/tupc2022_b
 *
 * tags: #貪欲法
 *
 * N 日ぶんは計算できないため、積雪日程M から求める. N<=10^12, M<=10^5
 *
 * 今日から次の日程を含まない前日までの日数は na-a で求められて
 * それまでに融雪することを考慮した、地下鉄の利用回数は sum-k 回である これらのmin が次の日程までの利用回数
 *
 * 次の日程までの積雪量は、単純に期間分だけsum から差し引く
 *
 * 最終降雪日m から最終日n までの期間の利用回数の計算を忘れない
 * また最終日は期間の変数t に含める
 *
 *
 */
use library::{max, min};

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: isize,
        ab: [(usize, isize); m]
    }

    let mut count = 0; // 登校できる日数
    let mut sum = 0; // i 日目を終えた時点での積雪量

    // a 日目に、b だけ降雪する
    for i in 0..m - 1 {
        let (a, b) = ab[i];
        sum += b;

        let (na, nb) = ab[i + 1]; // 次の積雪予定
        let t = na - a; // 次の日の前日までの日数（次の日程は含めない）
        let nc = min!(max!(0, sum - k), t as isize);

        count += nc;
        sum = max!(0, sum - t as isize); // 融雪した時の加減は0
    }

    // 最終融雪日から最後のn 日目までの登校分を計算
    let (a, b) = ab[m - 1];
    sum += b;
    let t = n - a + 1; // 最終日も含ませる
    let nc = min!(max!(0, sum - k), t as isize);
    count += nc;

    println!("{}", count);
}
