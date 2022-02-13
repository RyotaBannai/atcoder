/**
 * 01 部分和問題
 * https://algo-method.com/tasks/309
 *
 * i 番目の数値を使った時に数値 N (1<=i<=N) を作ることがきるか bool の DP を使って解く.
 */

#include <iostream>
using namespace std;

auto main() -> int
{
  // 入力
  int n, A;
  cin >> n >> A;

  bool dp[n + 1][A + 10]; // 多めにとる
  memset(dp, false, sizeof(dp));
  dp[0][0] = true; // 忘れない

  int as[A + 1];
  for (int i = 0; i < n; ++i) {
    cin >> as[i];
  }

  for (int i = 0; i < n; i++) {
    for (int a = 0; a <= A; a++) {
      // 前の状態を無条件に引き継ぐため条件の中に入れない
      dp[i + 1][a] |= dp[i][a]; // True を上書きしない

      if (as[i] <= a) {
        dp[i + 1][a] |= dp[i][a - as[i]]; // True を上書きしない
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

  cout << (dp[n][A] ? "Yes" : "No") << endl;
}