/**
 * 部分和数え上げ問題
 * https://algo-method.com/tasks/310
 *
 * i 番目の数値を使った時に数値 N (1<=i<=N) を作ることがきるか数値を引いた位置が 0
 * 以外なら、それを今回の index 加えることで n 通りできると考える. これに i-1
 * 回目でも部分和を作ることができるかもチェックし、それらを加算
 *
 * 同じ列を見ているわけではないため、01
 */

#include <iostream>
using namespace std;
const int MOD = 1000000009;

auto main() -> int
{
  // 入力
  int n, A;
  cin >> n >> A;

  int_least32_t dp[n + 1][A + 10]; // 多めにとる
  memset(dp, false, sizeof(dp));
  dp[0][0] = true; // 忘れない

  int as[A + 1];
  for (int i = 0; i < n; ++i) {
    cin >> as[i];
  }

  for (int i = 0; i < n; i++) {
    for (int a = 0; a <= A; a++) {
      // 前の状態を無条件に引き継ぐため条件の中に入れない
      (dp[i + 1][a] += dp[i][a]) %= MOD;

      if (as[i] <= a) {
        (dp[i + 1][a] += dp[i][a - as[i]]) %= MOD; // True を上書きしない
      }
    }
  }

  // debug
  // for (int i = 0; i <= n; i++) {
  //   for (int a = 0; a <= A + 1; a++) {
  //     cout << dp[i][a] << " ";
  //   }
  //   cout << endl;
  // }

  cout << dp[n][A] << endl;
}