use indexmap::indexmap;
use proconio::{fastout, input};
/**
 * Route Map
 *
 * https://atcoder.jp/contests/abc236/tasks/abc236_c
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String;n], // 普通
        t: [String;m], // 急行
    }

    let mut m = indexmap! {};
    s.iter().chain(t.iter()).for_each(|x| {
        if let Some(v) = m.get_mut(&x) {
            *v += 1;
        } else {
            m.insert(x, 1);
        }
    });

    for (_, v) in m {
        println!("{}", if v == 2 { "Yes" } else { "No" });
    }
}
