use proconio::{fastout, input, marker::Chars};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<String, usize>;

/**
 * (∀x∀)
 *
 * https://atcoder.jp/contests/abc242/tasks/abc242_e
 *
 * 最大の半分の長さのパターンをもつ Map 作って、左から半分（偶数なら半分、奇数なら中央）までの長さをキーで取り出す？
 * 事前に作るやり方だと 26^1_000 の組み合わせでだめ
 *
*/

#[fastout]
fn main() {
    input! { t: usize }
    let mut v = vec![Map::new(); 1_001];
    let mo = 998244353;

    let a = b'A';
    let mut keys = vec![];
    for i in 0..26 {
        keys.push(((a + i as u8) as char).to_string());
    }

    for i in 0..26 {
        v[1].insert(keys[i].clone(), i + 1);
    }

    // 2 桁
    for i in 2..=1_000 {
        let mut acc = 0;
        let mut m = Map::new();
        for j in 0..26 {
            for (k, (key, &x)) in v[i - 1].iter().enumerate() {
                if j == 0 && k == 0 {
                    acc += x;
                } else {
                    acc += 1;
                }

                acc %= mo;
                m.insert(keys[j].clone() + key, acc);
            }
        }
        v[i] = m;
    }

    // println!("{:?}", &v[2].iter().take(100).collect::<Vec<_>>());
    for _ in 0..t {
        input! {
            n: usize,
            s: String
        }
        let len = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
        let key: String = s.chars().take(len).collect();
        // println!("{}", key);
        println!("{}", v[len].get(&key).unwrap());
    }
}
