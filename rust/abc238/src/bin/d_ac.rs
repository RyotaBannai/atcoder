/**
 * AND and SUM
 *
 * https://atcoder.jp/contests/abc238/tasks/abc238_d
 *
 * 気持ち
 * ・直接 x y を求めずに、与えられている条件だけを満たす数値全体を検討
 * ・a は x y の & だから、2*a を s から引くと、1 が立っている部分が 0 になる残りができる
 * ・この残りは、x y の合計から a * 2 を引いたものだから、これらを & で 1 になった桁「以外の桁」は、どちらも 0 or 片方が 1 で片方が 0 . どちらのパターンでも s - a * 2 になるような 01 を切り替えて調整できる。
*/
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        qs: [(isize, isize); n],
    }
    for (a, s) in qs {
        let rest = s - 2 * a;
        let ans = if rest >= 0 && rest & a == 0 {
            "Yes"
        } else {
            "No"
        };
        println!("{}", ans);
    }
}
