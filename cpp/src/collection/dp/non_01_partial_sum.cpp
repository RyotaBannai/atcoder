/**
 * non 01 部分和問題
 * https://algo-method.com/tasks/313
 *
 * n 種類の品物が　mi（0<=i<=n）個ずつある時に総数 A にすることができるかどうか判定
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
  int ac[A + 1];
  for (int i = 0; i < n; ++i) {
    cin >> as[i] >> ac[i];
  }

  for (int i = 0; i < n; i++) {
    for (int a = 0; a <= A; a++) {
      // 前の状態を無条件に引き継ぐため条件の中に入れない
      dp[i + 1][a] |= dp[i][a]; // True を上書きしない

      if (as[i] <= a) {
        // 残りのカウントが残っていて、かつ引いた分が作れるのであれば
        if (ac[i] > 0 && dp[i + 1][a - as[i]]) {
          if (dp[i][a]) {}
          dp[i + 1][a] = true; // True を上書きしない
          ac[i]--;
        }
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