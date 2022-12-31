use num_integer::Roots;
use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
 * E - Fraction Floor Sum
 *
 * https://atcoder.jp/contests/abc230/tasks/abc230_e
 *
 * tags: #sum #floor
 *
 *
 * 'n を割った結果が n の平方根以下になる区間を n から差し引くと、
 * 残りは n の平方根程度になる'性質を利用する.
 *
 * 例えば、n=8
 * [√8] = 2 ただし [x] は x 以下の最大の整数を表わす.
 * 8 を割った結果が平方根以下(ここで 2 以下)となる数を n(=8)から差し引くと、
 * 残りは 2 以下程度となる.
 *
 * Σ[N/i] = 8/1 8/2 8/3 8/4 8/5 8/6 8/7 8/8
 *      = 8   4   2   2   1   1   1   1  
 *      = 8 + 4 + 2 * 2 + 1 * 4
 *
 *
 * n を割った結果が n の平方根をなる区間を探すには二分探索を使うと
 * この処理には O(logN*√N)
 *
 * 残りの割った結果が 平方根より多い区間は、
 * N の平方根以下となるため愚直に計算してO(√N)
 *
 *
 * 考察の手引き
 * まずは、N を程よく大きく設定した時に、
 * どういう計算結果が現れるのかを観察する. N=100 のようなケースが良い.
 *
 * それから、計算結果が同じになる区間を一度に計算できないか、
 * できるならば、どれくらいの範囲一度に計算できるかを確認.
 *
 * 一度に計算できる区間以外の部分の計算量はどれくらいか？ それは10^6 程度に収まるか？
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut sum = 0;
    let (mut l, mut r) = (1, n);
    let mut up = n;
    // println!("l{}, r {}", l, r);
    // println!("{}", sum);
    for i in 1..=n.sqrt() as usize {
        while r - l > 2 {
            let m = (r + l) / 2;
            if n / m > i {
                // m は小さすぎる
                l = m;
            } else {
                r = m;
            }
        }

        // println!("l {}, r {}", l, r);
        let mut mi = std::usize::MAX;
        for j in l..=r {
            if n / j == i {
                mi = mi.min(j);
            }
        }

        sum += (up - mi + 1) * i;

        // println!("mi {}, up {}", mi, up);

        r = mi - 1;
        l = 1;
        up = r;
    }

    for i in 1..=up {
        sum += n / i;
    }

    println!("{}", sum);
}
