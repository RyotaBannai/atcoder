use std::cmp::max;
use std::collections::BTreeMap as Map;
/**
 * Just K
 * https://atcoder.jp/contests/abc249/tasks/abc249_c
 *
 * AC
 *
 * dfs の代わりに、文字列を使うか使わないかの２択の全通り 2^n を数の bit で考える
 */

type M = Map<char, usize>;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        ss: [String; n],
    };

    let calc = |m: &mut M| {
        let mut ret = 0_usize;
        for &v in m.values() {
            if v == k {
                ret += 1;
            }
        }
        ret
    };

    let mut ans = 0;
    for mut x in 0..=(1 << n) {
        // dbg!(format!("{:#010b}", &x));
        let m = &mut Map::new();
        let mut i = 0;
        while i < n {
            if (x & 1) == 1 {
                ss[i].chars().for_each(|x| match m.get_mut(&x) {
                    Some(v) => *v += 1,
                    None => {
                        m.insert(x, 1);
                    }
                });
            }
            x >>= 1; // 初めのを使った後に右にシフト
            i += 1;
        }

        // dbg!(&m);
        ans = max(ans, calc(m));
    }

    println!("{}", ans);
}
