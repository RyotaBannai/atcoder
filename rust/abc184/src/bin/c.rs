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
use std::collections::{BinaryHeap, VecDeque};

// #[fastout]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }
    if a == c && b == d {
        println!("0");
        return;
    }

    let mut q = VecDeque::new();
    q.push_back((c, d, 0));
    while let Some((y, x, times)) = q.pop_front() {
        if times > 4 {
            continue;
        }
        println!("y,x {},{}", y, x);

        // 1. or 2. で完了
        if a + b == y + x || a - b == y - x {
            println!("{}", times + 1);
            return;
        }
        // あと一回の操作で完了
        if (a - y).abs() + (b - x).abs() <= 3 {
            println!("{}", times + 1);
            return;
        }

        let dif = y - x;

        for i in 0..=3 {
            for j in 0..=3 - i {
                if dif >= 0 {
                    q.push_back((dif / 2 + i, -(dif / 2 + j), times + 1));
                } else {
                    q.push_back((-(dif / 2 + i), dif / 2 + j, times + 1));
                }
            }
        }
    }
}
