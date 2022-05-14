use proconio::{fastout, input};
use std::cmp::min;

/**
 * Tahakashi and Animals
 * https://atcoder.jp/contests/abc251/tasks/abc251_e
 *
 * ・使う、使わないの状態を set で管理すると、その時点での状態を複製しないといけないため時間がかかる
 *   → DP を使う
 *
 * ・使う、使わないの状態を複配列の一番最後の配列で管理 0=使わない、1=使う
 * ・値は最小コスト
 *
 * ・全体感：
 * 　1. 全ての動物に餌をあげたい (どの動物にもあげない状態が生まれないようにする)
 * 　   具体的には、行動１ も行動 2 もしないと、動物2 に餌をあげられないことになる（→ 2回連続であげないことができない）
 * 　   → 先頭から処理をするならば、今回の行動が「あげない」であれば、前回は「あげる」行動でなければいけない
 *
 * 　（今回が「あげる」なら、前回がどちらの行動でもokで、重複してあげてもストが最小なれば良いから気にしない）
 *
 * 　2. 先頭から　「あげる」・「あげない」の処理をすると、行動１も行動N もあげない場合に、動物１に餌をあげないことになるため、行動１を分けて考える
 *
*/

#[fastout]
fn main() {
    input! {
        n: usize,
        cs: [usize; n],
    }

    let mut dp = vec![vec![0usize; 2]; n];

    // 行動１をしない場合 -> 行動２は必須
    for (i, c) in cs.iter().skip(1).enumerate() {
        if i != 0 {
            dp[i + 1][0] = dp[i][1];
        } else {
            dp[i + 1][0] = std::usize::MAX;
        }
        dp[i + 1][1] = min(dp[i][1], dp[i][0]) + c;
    }

    let mut ans = dp[n - 1][1];

    // 行動１をする場合
    dp[0][0] = std::usize::MAX;
    dp[0][1] = cs[0];
    for (i, c) in cs.iter().skip(1).enumerate() {
        dp[i + 1][0] = dp[i][1];
        dp[i + 1][1] = min(dp[i][1], dp[i][0]) + c;
    }

    ans = min(ans, min(dp[n - 1][0], dp[n - 1][1]));

    println!("{}", ans);
}
