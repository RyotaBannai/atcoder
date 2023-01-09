/**
 * @cpg_dirspec eleven_lover
 *
 * cpg run -p src/bin/other/aoj/eleven_lover_dp.rs
 */
use library::utils::{conv::*, read::*};

/**
 * Problem H: Eleven Lover
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/2182
 *
 * tags: #dp #動的計画法 #整数 #各桁で加える
 *
 */

fn main() {
    loop {
        let a = read::<String>()[0].chars().collect::<Vec<_>>();
        if a[0] == '0' {
            break;
        }
        let len = a.len();
        let mut dp = vec![vec![0; 11]; len + 1]; // 桁数分確保して、内側のvec は mod11 だから 0~10 までを確保
        let mut ans = 0;

        for (i, c) in a.iter().enumerate() {
            let n = toi(*c);
            if n != 0 {
                // 無条件に加える:= この桁(i) が先頭に使われるケースを想定.
                // 1234 であれば、１を無視して、2を先頭として検討するとき、2%11=2 となるケースとして、do[i+1][2] の組み合わせは１増える.
                // ただし 0 が先頭にくることはないことに注意
                dp[i + 1][n % 11] += 1;
            }

            // i 回目をみた後に、i+1 回目で使うn を１の桁として考えた時にできる組み合わせを考える
            for j in 0..11 {
                let key = j * 10 + n; // j は i 番目の数値でmod11 された数字
                dp[i + 1][key % 11] += dp[i][j]; // 組み合わせを増やす
            }

            // 各桁を見た時（i番目の桁）の11 でちょうど割り切れた数の組み合わせ([0])
            // 最後にdp[len][0] を取り出すのではなく、各回で取り出す. これは一度使い始めた桁が以降の桁全てを使わないといけない訳ではないため
            // 例えば、1111 で、１桁目を考慮したときに、最後の４桁目までの結果を待つ必要ない（部分列も検討している）
            // すなわち, 2 桁目を見た時に11 となってmod11=0 となるから、ここでその組みを解として保持する.
            // この組み合わせは3,4 桁以降も組み合わせとして考慮されるが、必ずしもmod11=0 になるとは限らない
            ans += dp[i + 1][0];
        }
        println!("{}", ans);
    }
}
