use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Snuke Festival
 *
 * https://atcoder.jp/contests/arc084/tasks/arc084_a
 *
 * tags: #中部全探索
 *
 * RE 10^9 は終わらない?... か、「メモリ制限: 256 MB」
*/

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mk = 1_000_000_002;
    let (mut va, mut vb, mut vc) = (vec![0; mk], vec![0; mk], vec![0; mk]);

    for (v, acc) in vec![(&a, &mut va), (&b, &mut vb), (&c, &mut vc)] {
        for x in v {
            acc[*x] += 1;
        }
    }

    for acc in vec![&mut va, &mut vc] {
        for i in 0..acc.len() - 1 {
            acc[i + 1] += acc[i];
        }
    }

    // dbg!(&va);
    // dbg!(&vc);

    // 中部（B）は、上部（A）より大きく、下部（C）より小さい
    let mut count = 0;
    for x in b {
        count += va[x - 1] * (n - vc[x]);
    }

    println!("{}", count);
}
