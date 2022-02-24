/**
 * 最小コスト弾性マッチング問題（動的時間伸縮法)）
 *
 * https://algo-method.com/tasks/316
 *
 * ルール
 * 各要素は異なる系列の要素とマッチング
 * マッチングは交差できない
 *
 * 考察
 * N 個のコストのうち n 番目と M を全てチェックして、最小コストを調べる
 * 一番初めは 1 番目だけでチェックするため単なる総和になる
 * 行方向に最小コストが更新される =
 * 一つの要素からの到達が異なる系列の多くの要素への最短コストとなる.
 */

#include <iomanip> // std::setw(int), std::setfill(char)
#include <ios>     // std::left, std::right
#include <iostream>
using namespace std;
using ll = long long;
static const int HUGE = 1 << 29;

auto main() -> int
{
  // 入力
  int N, M;
  cin >> N >> M;

  ll c[N + 1][M + 1];
  ll dp[N + 1][M + 1];
  for (int n = 0; n <= N; ++n)
    for (int m = 0; m <= M; ++m)
      dp[n][m] = HUGE;

  for (int n = 0; n < N; ++n) {
    for (int m = 0; m < M; ++m) {
      cin >> c[n][m];
    }
  }

  dp[0][0] = 0;

  for (int n = 0; n <= N; ++n) {
    for (int m = 0; m <= M; ++m) {
      dp[n + 1][m + 1] = min(dp[n][m], min(dp[n + 1][m], dp[n][m + 1])) + c[n][m];
    }
  }

  cout << dp[N][M] << endl;

  // debug
  /**
      5 5
      1 1 10 10 10
      10 1 1 10 10
      10 1 10 10 10
      10 1 1 1 10
      10 10 1 1 1
   */
  for (int n = 0; n < N; ++n) {
    for (int m = 0; m < M; ++m) {
      cout << c[n][m] << " ";
    }
    cout << endl;
  }

  for (int n = 0; n <= N; ++n) {
    for (int m = 0; m <= M; ++m) {
      cout << std::right << std::setw(20) << dp[n][m];
    }
    cout << endl;
  }
}