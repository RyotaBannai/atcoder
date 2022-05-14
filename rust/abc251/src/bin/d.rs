use proconio::{fastout, input};
/**
 * At Most 3 (Contestant ver.)
 * https://atcoder.jp/contests/abc251/tasks/abc251_d
 *
 * 組み合わせが、w 以下になるような錘の組み
 * 最低 3 種類以下の異なる数を用意？
 *
 * ・組み合わせて w になるようなおもり
 * ・個数は N 個以下
 * ・一つの重さは 1 以上、10^6 以下
*/

#[fastout]
fn main() {
    input! { mut w: usize }

    let mut a = vec![];
    if w % 2 == 0 {
        a.push(2);
        w /= 2;
    } else {
        a.push(1);
    }
}
