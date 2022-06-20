use proconio::{fastout, input, marker::Chars};
use std::cmp::{
    max, min,
    Ordering::{Equal, Greater, Less},
};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 *
 * 001 - Yokan Party（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_a
 *
 * tags: #二分探索 #最大値の最小化
 *
 * 7 45
 * 2
 * 7 11 16 20 28 34 38
 * 0 7 11 16 20 28 34 38 45
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut a: [usize; n]
    }

    a.push(l);

    let cut_by_x = |x: usize| -> (usize, usize) {
        let mut acc = 0;
        let mut count = 0;
        let mut mi = std::usize::MAX;
        let mut pre = 0;
        for c in &a {
            acc += c - pre;
            pre = *c;
            if acc >= x {
                mi = min(mi, acc);
                count += 1;
                acc = 0;
            }
        }

        (count, mi)
    };

    let mut left = 1;
    let mut right = l;

    while right - left > 1 {
        // right, left の差分が１より大きい間（left, mid, right が 1 の差で並ぶまで探索を続ける）
        // 最小の最大を求めたいため、k+1 でカットできるのなら、left を更新して、より大おおきいカット幅(x)を求めていく。
        // もし更新して、新しい mid で k+1 でカットできないなら、right を更新して調整.
        // 判定ロジックが x をもとにカット数を決定しているため、二分探索の最終値が最大カット幅へ帰着することができる

        let mid = (right + left) / 2;
        let (cuts, _) = cut_by_x(mid);
        // dbg!(format!("{}, {}, {}", left, mid, right));
        // dbg!(cuts);

        // 同じなら left に入れるため、最後の答えも left を使う
        match cuts.cmp(&(k + 1)) {
            Less => right = mid,
            Equal | Greater => left = mid,
        }
    }

    println!("{}", left);
}
