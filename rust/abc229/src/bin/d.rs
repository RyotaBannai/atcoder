/**
 * Longest X
 *
 * https://atcoder.jp/contests/abc229/tasks/abc229_d
 *
 * 累積和
 * ・X をどこに埋めるかではなく、. が許容値 k を超えない範囲でチェック範囲を伸縮させながらテスト
 * ・while の条件は右を一つ試した時に条件を満たすかどうかを先にチェック
 * ・先に右が over しないかチェック
 */
use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;
fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    let mut cumsum = vec![0; s.len() + 1];
    for (i, &c) in s.iter().enumerate() {
        cumsum[i + 1] = if c == '.' { cumsum[i] + 1 } else { cumsum[i] }
    }
    let mut ans = 0_usize;
    let mut r = 0;
    for l in 0..cumsum.len() {
        while r < s.len() && cumsum[r + 1] - cumsum[l] <= k {
            r += 1;
        }
        ans = max(ans, r - l);
    }

    println!("{}", ans);
}
