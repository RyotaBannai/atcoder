use proconio::{fastout, input, marker::Chars};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;

/**
 * ABC Transform
 *
 * https://atcoder.jp/contests/abc242/tasks/abc242_d
 *
 * S0 = ABC
 * S1 = BC CA AB
 * S2 = CA AB AB BC BC CA
 *
 * 1 3 = C だけど、
 * 一つの文字が、x+1, x+2 の文字へ変換 (x+1 は偶数番目、x+2 は奇数番目)
 * 最終的に一番最初の派生元が分かれば、あとはルールに従って、変換して行けば良い
 * ・最終的な派生先？
 * 　S0 （この場合 t==0）か、k==0 になる位置を探す？ k==0 になる t から、変換をすればいい
 *
 * 2 5 ?
 *
 * 2 2 ?
 * t=2, k=1
 * t=1, k=0
 *
*/

fn convert(c: char, n: usize) -> char {
    (b'A' + (c as u8 - b'A' + n as u8) % 3) as char
}

#[derive(new)]
struct Rec<'a> {
    s: &'a [char], // String chars().nth(n) は使わない..
}
impl<'a> Rec<'a> {
    fn find_char(&self, t: usize, k: usize) -> char {
        if t == 0 {
            // S0 で ABC へ戻った
            self.s[k]
        } else if k == 0 {
            // t!=0 で文字列の先頭に到達した
            // 与えられた文字列の先頭から、t 回返還された結果が欲しい
            convert(self.s[0], t % 3)
        } else {
            // まだ派生元に到達していない
            // 一つ前の文字列 t - 1 を見て、親の index k/2 へ移動
            let c = &self.find_char(t - 1, k / 2);

            if k % 2 == 0 {
                // 自分が偶数なら 親+1
                convert(*c, 1)
            } else {
                // 奇数なら 親+2
                convert(*c, 2)
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        s: Chars,
        n: usize,
        qs: [(usize, usize); n]
    }

    let rec = Rec::new(&s);
    for (t, k) in qs {
        let ans = rec.find_char(t, k - 1); // 問題の 1 番目は文字列の 0 番目
        println!("{}", ans);
    }
}
