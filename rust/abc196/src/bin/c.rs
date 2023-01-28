use itertools::Itertools;
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
use library::utils::conv::{calc_num, toi};

// #[fastout]
fn main() {
    input! {
        mut cs: Chars
    }
    let n = calc_num(&cs);
    cs = cs.into_iter().rev().collect_vec();
    let mut len = 0;
    let mut m = n;
    while m > 0 {
        len += 1;
        m /= 10;
    }

    let mut ans = 0;
    let mut d = 2usize;
    loop {
        // 今回作った偶数桁がn を越えた.
        if d > len {
            break;
        }
        let low = 10usize.pow((d as u32 / 2) - 1);
        // n の前半部分の最大値と比較した最小
        if d < len {
            // 前半部分の最大値
            let mut up = 0;
            for i in 0..d / 2 {
                up += 9 * 10usize.pow(i as u32);
            }
            ans += up - low + 1;
        } else {
            let fst = calc_num(&cs[d / 2..d].iter().cloned().rev().collect_vec());
            let sec = calc_num(&cs[0..d / 2].iter().cloned().rev().collect_vec());
            let up = fst.min(sec);
            if up >= low {
                // 10,000,000 の時
                // 0 - 1,000 となる.
                // 他にも 1000,0999 とかもだめ.
                ans += up - low + 1;
            }
        }

        d += 2;
    }
    println!("{}", ans);
}
