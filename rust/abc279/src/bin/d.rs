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

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64
    }

    let mut m = 2usize;
    let g = 1.;

    let mut prev = a / g; // 魔法を 0 回使う
    loop {
        // println!("prev in {}, m {}", prev, m);
        let nx = m as f64 * b + (a / (g + m as f64).sqrt());
        if nx > prev {
            break;
        }
        prev = nx;
        m = m.saturating_mul(m);
    }
    let mut r = m as usize;
    let mut l = 0;

    // println!("prev {:.7}", prev);
    // println!("l,r {},{}", l, r);
    // while r - l >= 1 {
    let mut cnt = 0;
    loop {
        cnt += 1;
        if cnt > 50 {
            break;
        }
        println!("l,r {},{}", l, r);
        let mid = (r.saturating_add(l)) / 2;
        let nx = mid as f64 * b + (a / (g + mid as f64).sqrt());
        if nx == prev {
            // break;
        }
        if nx > prev {
            r = mid;
        } else {
            l = mid;
        }
        // println!("l,r {},{}", l, r);

        prev = nx;
    }
    println!("{:.10}", prev);
}
