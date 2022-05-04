use proconio::{fastout, input};
/**
 * Happy New Year!
 *
 * https://atcoder.jp/contests/abc233/tasks/abc233_c
 *
 * または 数値を bitset で表してから、1 を 2 に置き換える
 *
 */
#[fastout]
fn main() {
    input! {
        mut k: usize
    }

    let mut ans = "".to_string();
    while k > 0 {
        if k % 2 == 0 {
            ans.push('0'); // 割り切れるなら 1 が立っていない
        } else {
            ans.push('2');
        }
        k /= 2; // 二進数だから 2 で割ることで桁を下げる
    }
    println!("{}", ans.chars().rev().collect::<String>());
}
