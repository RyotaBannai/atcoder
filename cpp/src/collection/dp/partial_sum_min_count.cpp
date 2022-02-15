/**
 * 最小個数部分和問題
 * https://algo-method.com/tasks/311
 *
 * n 個の正の整数 a[0],a[1],…,a[n−1] と正の整数 A が与えられる。
 * これらの整数から何個かの整数を選んで総和が A にする方法をすべて考えた時、
 * 選ぶ整数の個数の最小値を求めよ。
 * A にすることができない場合は -1 と出力せよ。
 *
 */

#include <iostream>
using namespace std;

auto main() -> int
{
  // 入力
  int n, A;
  cin >> n >> A;

  int dp[n + 1][A + 10]; // 多めにとる
  memset(dp, false, sizeof(dp));
  dp[0][0] = true; // 忘れない

  int as[A + 1];
  for (int i = 0; i < n; ++i) {
    cin >> as[i];
  }

  for (int i = 0; i < n; i++) {
    for (int a = 0; a <= A; a++) {
      // いったん前の状態を無条件に引き継ぐため条件の中に入れない
      dp[i + 1][a] += dp[i][a];

      if (as[i] <= a) {
        dp[i + 1][a] += min(dp[i + 1][a], dp[i][a - as[i]] + 1);
      }
    }
  }

  // debug
  for (int i = 0; i <= n; i++) {
    for (int a = 0; a <= A + 1; a++) {
      cout << dp[i][a] << " ";
    }
    cout << endl;
  }

  cout << (dp[n][A] ? dp[n][A] : -1) << endl;
}