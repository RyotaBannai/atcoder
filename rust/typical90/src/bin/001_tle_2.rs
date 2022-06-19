use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
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
* 001 - Yokan Party（★4）
*
* https://atcoder.jp/contests/typical90/tasks/typical90_a
*
* BinSearch Ref:
* https://github.com/hatoo/competitive-rust-snippets/blob/master/src/binary_search.rs
*
* TLE この二分探索じゃない..
*
*/
use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T, begin: usize) -> usize;
    // fn upper_bound(&self, x: &T, begin: usize) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T, begin: usize) -> usize {
        let mut low = begin;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    // fn upper_bound(&self, x: &T, begin: usize) -> usize {
    //     let mut low = 0;
    //     let mut high = self.len();

    //     while low != high {
    //         let mid = (low + high) / 2;
    //         match self[mid].cmp(x) {
    //             Ordering::Less | Ordering::Equal => {
    //                 low = mid + 1;
    //             }
    //             Ordering::Greater => {
    //                 high = mid;
    //             }
    //         }
    //     }
    //     low
    // }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n]
    }

    let lc = {
        let mut mi = a[0];
        for i in 0..n - 1 {
            mi = min(mi, a[i + 1] - a[i]);
        }
        mi = min(mi, l - a[n - 1]);
        mi
    };

    // 長さ 1 から初めて l までの間に、l の長さ x 以上で k 回切れるか、と考える
    let mut ma = std::usize::MIN;
    for x in lc..=l / k {
        let mut ok = true;
        let mut prev_pos = 0;
        let mut next_pos = x;
        let mut local_mi = std::usize::MAX;
        let mut last_i = 0_usize;
        for i in 1..=k {
            // k 回切れるかどうかチェック
            let pos = a.lower_bound(&next_pos, last_i);
            if pos == n {
                ok = false;
                break;
            } else {
                local_mi = min(local_mi, a[pos] - prev_pos);
                prev_pos = a[pos]; // 切れ目を入れる
                next_pos = prev_pos + x; // x 以上で切れるか試しているため、前の切れ目から x 以上の位置が次の lower_bound の位置になる
                last_i = pos;

                // 最後のカットなら、末尾との距離も判定
                if i == k {
                    if l - a[pos] < x {
                        ok = false;
                    } else {
                        local_mi = min(local_mi, l - a[pos]);
                    }
                }
            }
        }

        if ok {
            ma = max(ma, local_mi);
        }
    }

    println!("{}", ma);
}
