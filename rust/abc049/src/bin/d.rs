use im_rc::HashSet;
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
use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use library::structure::disjoint_set::*;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * D - 連結
 *
 * https://atcoder.jp/contests/abc049/tasks/arc065_b
 *
 *
 * TLE
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        l: usize,
        pq: [(usize, usize); k],
        rs: [(usize, usize); l],
    }

    let mut ufpq = DisjointSetGroups::new(n);
    let mut ufrs = DisjointSetGroups::new(n);
    for (p, q) in pq {
        if !ufpq.same(p, q) {
            ufpq.merge(p, q);
        }
    }

    for (r, s) in rs {
        if !ufrs.same(r, s) {
            ufrs.merge(r, s);
        }
    }

    let mut count = vec![1; n + 1];
    let mut q = BTreeSet::from_iter(1..=n);
    loop {
        if q.is_empty() {
            break;
        }
        if let Some(&u) = q.iter().next() {
            q.remove(&u);

            let ur1 = ufpq.find(u); // 道路
            if let Some(s1) = ufpq.comps.get(&ur1) {
                let mut c = 0;
                let mut m = BTreeMap::new();
                for x in s1.iter() {
                    let e = m.entry(x).or_insert(0);
                    *e += 1;
                    if *e >= 2 {
                        c += 1;
                    }
                }

                let mut ss = s1.clone();
                loop {
                    if ss.is_empty() {
                        break;
                    }
                    if let Some(&y) = ss.iter().next() {
                        q.remove(&y);
                        ss.remove(&y);
                        let mut cc = c;
                        let mut mm = m.clone();
                        let ur2 = ufrs.find(y); // 鉄道
                        if let Some(s2) = ufrs.comps.get(&ur2) {
                            for x in s2 {
                                let e = mm.entry(x).or_insert(0);
                                *e += 1;
                                if *e >= 2 {
                                    cc += 1;
                                }
                            }

                            for (&k, &v) in mm.iter() {
                                if v >= 2 {
                                    q.remove(k);
                                    ss.remove(k);
                                    count[*k] = cc;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // println!("comps");
    // println!("{:?}", ufpq.comps);
    // println!("{:?}", ufrs.comps);

    // 1-index だから0 は飛ばす
    for x in count.iter().skip(1) {
        print!("{} ", x);
    }
}
