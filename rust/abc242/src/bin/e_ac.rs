use proconio::{fastout, input, marker::Chars};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;

/**
 * (∀x∀)
 *
 * https://atcoder.jp/contests/abc242/tasks/abc242_e
 *
 *
 * 1. 回文だから、まず中央の文字までチェックすれば良い
 * 2. １. で前半分と後ろ半分が同じ前提で考える（abcba であって、abcaa は考えない）
 * 3. 2. の場合で、全半部分が後半部分より小さいときに限り、最後 -1 する. これは abcba と考えたときに、abcaa よりちょうど一つ多く数えていることからわかる
 *
 * 数え上げの考え方
 * 先頭の桁から数えて、次の桁でとりうる文字（数字）をかけるやり方がらく
 *
 *
 * 参考
 * https://atcoder.jp/contests/abc242/editorial/3516
 *
*/

#[fastout]
fn main() {
    input! { t: usize }
    let mo = 998244353;

    for _ in 0..t {
        input! {
            n: usize,
            s: String
        }
        let mut sum = 0;
        s.chars().take(((n - 1) / 2) + 1).for_each(|c| {
            // dbg!(c);
            sum *= 26;
            sum %= mo;
            sum += (c as u8 - b'A') as usize;
            sum %= mo;
        });
        sum += 1; // 最後の桁は次の桁の計算に関与しないため +1
        sum %= mo;
        let (mut p, mut q) = (0, s.len() - 1);
        let mut target = s.chars().collect::<Vec<char>>();
        while p < q {
            target[q] = target[p];
            p += 1;
            q -= 1;
        }

        if s < target.iter().collect::<String>() {
            sum += mo - 1; // これ厳しい... 0 の状態で -1 しとき、本来であれば mo になるはずだけど, 0-1 では不正解
            sum %= mo;
        }

        println!("{}", sum);
    }
}
