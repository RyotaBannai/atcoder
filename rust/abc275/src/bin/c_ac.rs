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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<(
    (isize, isize),
    (isize, isize),
    (isize, isize),
    (isize, isize),
)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

#[fastout]
fn main() {
    input! {
        t: [Chars; 9],
    }
    let mut ans = Set::new();
    // 探索の際の固定位置(左上(?)の頂点)
    for i in 0..9 {
        for j in 0..9 {
            // 考えうる次の頂点、時計回り
            // 4頂点が一点になってはいけないから、dx!=dy
            for dy in 0..9 {
                for dx in 0..9 {
                    if dx == dy && dx == 0 {
                        continue;
                    }
                    let p1 = (j as isize, i as isize); // (x,y)
                    let p2 = (p1.0 + dx as isize, p1.1 + dy as isize);
                    let p3 = (p2.0 - dy as isize, p2.1 + dx as isize); // 注意
                    let p4 = (p3.0 - dx as isize, p3.1 - dy as isize);

                    // if p1 == (0, 0) {
                    //     println!("{:?}", [p1, p2, p3, p4]);
                    //     panic!("");
                    // }

                    if [p1, p2, p3, p4].iter().all(|&(x, y)| {
                        x >= 0 && x < 9 && y >= 0 && y < 9 && t[y as usize][x as usize] == '#'
                    }) {
                        // println!("{:?}", [p1, p2, p3, p4]);
                        let mut s = vec![p1, p2, p3, p4];
                        s.sort_unstable();
                        ans.insert((s[0], s[1], s[2], s[3]));
                    }
                }
            }
        }
    }

    println!("{}", ans.len());
}
