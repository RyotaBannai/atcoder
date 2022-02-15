/**
 * 最小個数部分和問題
 * https://algo-method.com/tasks/311
 *
 * n 個の正の整数 a[0],a[1],…,a[n−1] と正の整数 A が与えられる。
 * これらの整数から何個かの整数を選んで総和が A にする方法をすべて考えた時、
 * 選ぶ整数の個数の最小値を求めよ。
 * A にすることができない場合は -1 と出力せよ。
 *
 * 前回と同じ位置と、今回の品物を使った場合とで比較して最小の方を使って随時更新
 */

#include <iomanip> // std::setw(int), std::setfill(char)
#include <ios>     // std::left, std::right
#include <iostream>
#include <vector>
using namespace std;
static const int HUGE = 1 << 29;

auto main() -> int
{
  // 入力
  int n, A;
  cin >> n >> A;

  // https://atcoder.jp/contests/APG4b/tasks/APG4b_t
  vector<vector<int>> dp(n + 1, vector<int>(A + 10, HUGE));

  dp[0][0] = 0; // 忘れない

  int as[A + 1];
  for (int i = 0; i < n; ++i) {
    cin >> as[i];
  }

  for (int i = 0; i < n; i++) {
    for (int a = 0; a <= A; a++) {
      // いったん前の状態を無条件に引き継ぐため条件の中に入れない
      dp[i + 1][a] = dp[i][a];

      if (as[i] <= a) {
        dp[i + 1][a] = min(dp[i + 1][a], dp[i][a - as[i]] + 1);
      }
    }
  }

  // debug
  // for (int i = 0; i <= n; i++) {
  //   for (int a = 0; a <= A + 1; a++) {
  //     cout << std::right << std::setw(10) << dp[i][a];
  //   }
  //   cout << endl;
  // }

  cout << (dp.at(n).at(A) == HUGE ? -1 : dp[n][A]) << endl;
}