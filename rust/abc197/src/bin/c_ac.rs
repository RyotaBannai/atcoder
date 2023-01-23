use library::chmin;
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
use std::usize::MAX;

/**
 * ORXOR
 *
 * https://atcoder.jp/contests/abc197/tasks/abc197_c
 *
 * 毎数字ごとに仕切りを入れるか入れないかを選択する全探索.
 * 2^20 程度であれば、区間の計算をメモ化しなくても間に合う.
 *
 * 以下の実装では区間をメモ化しているが、なくても大丈夫だし、
 * 高速改善もない.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    if n == 1 {
        println!("{}", a[0]);
        return;
    }
    let mut ans = MAX;
    let mut mem = vec![vec![MAX; n]; n]; // 0-index
    let mut q = VecDeque::new();
    q.push_back((0, 1, vec![a[0]], 0)); // 一番初めの数値と単位元を入れておく
    q.push_back((1, 1, vec![], a[0])); // xor を使ったから st もen も1
    while let Some((st, en, mut xs, xor)) = q.pop_front() {
        if en == n {
            chmin!(ans, xor);
            continue;
        }

        xs.push(a[en]);

        // 区切らない場合
        if en != n - 1 {
            // 最後は区切るしかない
            q.push_back((st, en + 1, xs.clone(), xor));
        }

        // 区切る場合
        if mem[st][en] == MAX {
            let mut tmp = 0; // or の単位元
            for x in xs {
                tmp |= x;
            }
            mem[st][en] = tmp;
        };
        q.push_back((en + 1, en + 1, vec![], mem[st][en] ^ xor));
    }

    println!("{}", ans);
}
