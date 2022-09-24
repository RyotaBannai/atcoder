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
 * C - Simple path
 *
 * https://atcoder.jp/contests/abc270/tasks/abc270_c
 *
 * WA
 */
struct Rec {
    st: usize,
    end: usize,
    v: Vec<Vec<usize>>,
    p_s: Vec<usize>,
    p_e: Vec<usize>,
}
impl Rec {
    fn new(st: usize, end: usize, v: Vec<Vec<usize>>) -> Self {
        Self {
            st,
            end,
            v,
            p_s: vec![],
            p_e: vec![],
        }
    }
    fn find(&mut self, x: usize, path: Vec<usize>) {
        // println!("x {}", x);
        if self.st != 1 && x == self.st {
            self.p_s = path;
            self.p_s.push(x);
            return;
        } else if self.end != 1 && x == self.end {
            self.p_e = path;
            self.p_e.push(x);
            return;
        }
        // println!("{:?}", &self.v[x]);
        for y in self.v[x].clone() {
            let mut np = path.clone();
            np.push(x);
            self.find(y, np);
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        st: usize,
        end: usize,
        es: [(usize,usize); n-1] // 有
    }

    let mut v = vec![vec![]; n + 2];

    // 連接リストを作る
    for (s, e) in es {
        v[s].push(e);
    }

    let mut rec = Rec::new(st, end, v);
    rec.find(1, vec![]);

    // println!("{:?}", rec.p_s);
    // println!("{:?}", rec.p_e);

    if st == 1 {
        for x in rec.p_e.iter() {
            print!("{} ", x);
        }
    } else if end == 1 {
        for x in rec.p_s.iter().rev() {
            print!("{} ", x);
        }
    } else {
        let mut p_i = 0;
        for (i, (&x, &y)) in rec.p_s.iter().zip(&rec.p_e).enumerate() {
            if x != y {
                break;
            }
            p_i = i; // 共通の親の位置
        }
        for x in rec.p_s.iter().skip(p_i).rev() {
            print!("{} ", x);
        }
        for x in rec.p_e.iter().skip(p_i + 1) {
            print!("{} ", x);
        }
    }
    println!();

    // println!("{:?}", rec.p_e);
}
