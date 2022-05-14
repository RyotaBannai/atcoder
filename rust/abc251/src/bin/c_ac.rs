use proconio::{fastout, input};
use std::collections::BTreeMap;

type Map = BTreeMap<String, usize>;
/**
 * Poem Online Judge
 * https://atcoder.jp/contests/abc251/tasks/abc251_d
*/
#[fastout]
fn main() {
    input! {
        n: usize,
        x: [(String, usize); n], // 文字列, 得点
    }

    let mut m = Map::new();
    let mut max_i = 0;
    let mut max = 0usize;
    for (i, (s, t)) in x.iter().enumerate() {
        // オリジナル
        if m.get(s).is_none() {
            m.insert(s.to_string(), *t);
            if *t > max {
                max = *t;
                max_i = i;
            }
        }
    }

    println!("{}", max_i + 1);
}
