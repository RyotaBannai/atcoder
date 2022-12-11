/**
 * @cpg_dirspec range_query_2d
 *
 * cpg run -p src/bin/query/seg_tree/range_query_2d.rs
 */
// use proconio::{fastout, input, marker::Chars};
use std::io;
use std::isize::MAX;
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<Point>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 領域探索（kD 木）
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/all/DSL_2_C
 *
 * tags: #領域探索 #kd_tree
 *
 */

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    v: usize,
    x: isize,
    y: isize,
}

impl Point {
    fn new(v: usize, x: isize, y: isize) -> Self {
        Self { v, x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
    mid: usize,
    l: isize, // ord != mid
    r: isize, // ord != mid
}

impl Node {
    fn new(mid: usize, l: isize, r: isize) -> Self {
        Self { mid, l, r }
    }
}

struct Rec {
    np: isize,     // 行きがけ順
    t: Vec<Node>,  // 行きがけ順を index にもつ
    s: Vec<Point>, // 平面座標 の集合
    ans: Set,      // 解答
}
impl Rec {
    fn new(n: usize, s: Vec<Point>) -> Self {
        Self {
            t: vec![Node::new(0, 0, 0); n],
            np: -1,
            s,
            ans: Set::new(),
        }
    }

    // 二分探索木を構築
    // mid は中間地点
    // l,r は行きがけの ord で、0<=x<=n(集合の要素数)
    // n軸に対して、ソート->半分にスプリット->n+1軸に対して... を繰り返す
    // 頂点への参照を持つ代わりに ord で頂点を管理する.
    fn make_kd_tree(&mut self, l: usize, r: usize, depth: usize) -> isize {
        if l >= r {
            // 葉
            return MAX;
        };

        // 平面上で見る
        // 偶数回 x 座標、奇数回 y 座標でソート
        if depth % 2 == 0 {
            self.s[l..r].sort_by(|p1, p2| p1.x.cmp(&p2.x));
        } else {
            self.s[l..r].sort_by(|p1, p2| p1.y.cmp(&p2.y));
        }

        let mid = (l + r) / 2;
        self.np += 1;
        let ord = self.np as usize;
        self.t[ord].mid = mid;
        self.t[ord].l = self.make_kd_tree(l, mid, depth + 1);
        self.t[ord].r = self.make_kd_tree(mid + 1, r, depth + 1);
        ord as isize
    }

    fn find(&mut self, i: usize, sx: isize, tx: isize, sy: isize, ty: isize, depth: usize) {
        let u = self.t[i];
        let p = self.s[u.mid];
        let (x, y) = (p.x, p.y);
        if sx <= x && x <= tx && sy <= y && y <= ty {
            // レンジクエリの範囲内にある
            self.ans.insert(p);
        }

        let mut rec = |i: isize| self.find(i as usize, sx, tx, sy, ty, depth + 1);

        if depth % 2 == 0 {
            // 偶数回 x 座標について考える

            // x が下限より大きい場合は、左側を再帰的に探索
            if u.l != MAX && sx <= x {
                rec(u.l);
            }
            // x が上限より小さい場合は、右側を再帰的に探索
            if u.r != MAX && x <= tx {
                rec(u.r);
            }
        } else {
            // 奇数回 y 座標について考える

            // y が下限より大きい場合は、左側を再帰的に探索
            if u.l != MAX && sy <= y {
                rec(u.l);
            }
            // y が上限より小さい場合は、右側を再帰的に探索
            if u.r != MAX && y <= ty {
                rec(u.r);
            }
        }
    }
}

// #[fastout]
fn main() {
    let n = read::<usize>()[0];
    let mut s = vec![];
    for v in 0..n {
        let a = read::<isize>();
        let (x, y) = (a[0], a[1]);
        s.push(Point::new(v, x, y));
    }

    let mut qs = vec![];
    let q = read::<usize>()[0];
    for _ in 0..q {
        let b = read::<isize>();
        qs.push(((b[0], b[1]), (b[2], b[3]))); // ((sx, tx), (sy, ty))
    }

    let mut rec = Rec::new(n, s);
    rec.make_kd_tree(0, n, 0);

    // ans
    for ((sx, tx), (sy, ty)) in qs {
        rec.ans = Set::new();
        rec.find(0, sx, tx, sy, ty, 0); // 根(index=0, mid=ソート時の中央値)から始める
        for p in rec.ans {
            println!("{}", p.v);
        }
        println!();
    }
}
