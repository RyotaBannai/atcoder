/**
 * 最小コスト弾性マッチング問題
 *
 * https://algo-method.com/tasks/316
 *
 * ルール
 * 各要素は異なる系列の要素とマッチング
 * マッチングは交差できない
 */
#include <iostream>
using namespace std;

auto main() -> int
{
  // 入力
  int N, M;
  cin >> N >> M;

  int c[N + 1][M + 1];
  int dp[N + 1][M + 1];
  memset(dp, 0, sizeof(dp));

  for (int n = 0; n < N; ++n) {
    for (int m = 0; m < M; ++m) {
      cin >> c[n][m];
    }
  }

  for (int n = 0; n <= N; ++n) {
    for (int m = 0; m <= M; ++m) {
      dp[n + 1][m + 1] = min(dp[n][m], min(dp[n + 1][m], dp[n][m + 1])) + c[n][m];
    }
  }

  for (int n = 0; n <= N; ++n) {
    for (int m = 0; m <= M; ++m) {
      cout << dp[n][m];
    }
    cout << endl;
  }

  cout << dp[N][M] << endl;
}