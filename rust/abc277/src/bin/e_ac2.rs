use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
use maplit::hashmap;
use std::collections::{BTreeMap, BTreeSet, HashMap};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * E - Crystal Switches
 *
 * https://atcoder.jp/contests/abc277/tasks/abc277_e
 */

#[fastout]
fn main() {
    input! {
        n: usize, // 2<=
        m: usize, // 1<=
        k: usize, // 0<=k
        es: [(usize, usize, usize); m], // 無効グラフ, 通れる・通れないの初期状態
        s: [usize; k], // スイッチがおいてある頂点 1<=
    }
    let mut swi = HashMap::new();
    for x in s {
        swi.insert(x, true);
        swi.insert(x + n, true);
    }

    // 初期状態
    let mut st: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    for &e in &es {
        let (from, to, s) = e;
        let (from1, to1) = (from + n, to + n);
        if s == 1 {
            st.entry(from).or_insert(hashmap! {}).entry(to).or_insert(s);
            st.entry(to).or_insert(hashmap! {}).entry(from).or_insert(s);
        } else {
            // スイッチon の時
            st.entry(to1)
                .or_insert(hashmap! {})
                .entry(from1)
                .or_insert((s + 1) % 2);

            st.entry(from1)
                .or_insert(hashmap! {})
                .entry(to1)
                .or_insert((s + 1) % 2);
        }
    }

    let inf = std::usize::MAX;
    let mut d = vec![inf; n * 2 + 1]; // 距離 スイッチの切り替えの状態（偶奇）で分ける
    d[1] = 0;

    let mut x = VecDeque::new();
    x.push_back((0, 1)); // (d, u)

    while let Some(y) = x.pop_front() {
        let (ud, u) = y;
        if d[u] != ud {
            continue;
        }
        let nd = ud + 1;
        if let Some(cs) = st.get(&u) {
            for (&chi, &nst) in cs.iter() {
                if d[chi] > nd && nst == 1 {
                    d[chi] = nd;
                    x.push_back((nd, chi));
                }
            }
        }

        // スイッチの切り替えを考慮する
        if swi.get(&u).is_some() {
            let nu = if u <= n { u + n } else { u - n };
            if let Some(cs) = st.get(&nu) {
                for (&chi, &nst) in cs.iter() {
                    if d[chi] > nd && nst == 1 {
                        d[chi] = nd;
                        x.push_back((nd, chi));
                    }
                }
            }
        }
    }
    // println!("{:?}", &d);

    let mut ans = inf;
    ans = ans.min(d[n]);
    ans = ans.min(d[2 * n]);
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
