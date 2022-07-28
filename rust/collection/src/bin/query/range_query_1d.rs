/**
 * @cpg_dirspec range_query_1d
 *
 * cpg run -p src/bin/query/range_query_1d.rs
 */
// use proconio::{fastout, input, marker::Chars};
use std::io;
use std::usize::MAX;
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
    mid: usize,
    l: usize, // ord != mid
    r: usize, // ord != mid
}

impl Node {
    fn new(mid: usize, l: usize, r: usize) -> Self {
        Self { mid, l, r }
    }
}

struct Rec {
    np: isize,     // 行きがけ順
    t: Vec<Node>,  // 行きがけ順を index にもつ
    s: Vec<usize>, // 直線座標 の集合
    ans: Set,      // 解答
}
impl Rec {
    fn new(n: usize, s: Vec<usize>) -> Self {
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
    fn make_kd_tree(&mut self, l: usize, r: usize, depth: usize) -> usize {
        if l >= r {
            // 葉
            return MAX;
        };
        // [l,r) だけでソートしたい
        // [l,r) [2,4)
        // s [3,5,6,2,7]
        // v2 = [6,2,7] s = [3,5]
        // v3 = [7] v2 = [6,2]
        // v2.sort() [2,6]

        let mut v2 = self.s.split_off(l); // split: 後ろを全部返却
        let mut v3 = v2.split_off(r - l); // r は開区間
        v2.sort_unstable_by(|a, b| a.cmp(b)); // 直線上で見る
        self.s.append(&mut v2);
        self.s.append(&mut v3);
        let mid = (l + r) / 2;
        self.np += 1;
        let ord = self.np as usize;
        self.t[ord].mid = mid;
        self.t[ord].l = self.make_kd_tree(l, mid, depth + 1);
        self.t[ord].r = self.make_kd_tree(mid + 1, r, depth + 1);
        ord
    }

    fn find(&mut self, i: usize, sx: usize, tx: usize, depth: usize) {
        let u = self.t[i];
        let x = self.s[u.mid];
        if sx <= x && x <= tx {
            // レンジクエリの範囲内にある
            self.ans.insert(x);
        }

        // x が下限より大きい場合は、左側を再帰的に探索
        if u.l != MAX && sx <= x {
            self.find(u.l, sx, tx, depth + 1);
        }
        // x が上限より小さい場合は、右側を再帰的に探索
        if u.r != MAX && x <= tx {
            self.find(u.r, sx, tx, depth + 1);
        }
    }
}

// #[fastout]
fn main() {
    // 領域に含まれる要素を１行に半角空白区切りで出力せよ
    // 1 行目:　n 集合の要素数 q 領域探索のクエリ数
    // 2 行目: 要素数 n の集合が１行で与えられる
    // 3 行目~2+q 行目にクエリが与えられる

    let a = read::<usize>();
    let (n, q) = (a[0], a[1]);
    let s = read::<usize>(); // 集合
    let mut qs = vec![];
    for _ in 0..q {
        let b = read::<usize>();
        qs.push((b[0], b[1]));
    }

    let mut rec = Rec::new(n, s);
    rec.make_kd_tree(0, n, 0);

    // ans
    for (sx, tx) in qs {
        rec.ans = Set::new();
        rec.find(0, sx, tx, 0); // 根(index=0, mid=ソート時の中央値)から始める
        for x in rec.ans {
            print!("{} ", x);
        }
        println!();
    }
}
