/**
 * 最小個数部分和問題
 * https://algo-method.com/tasks/311
 *
 * n 個の正の整数 a[0],a[1],…,a[n−1] と正の整数 A が与えられる。
 * これらの整数から K 個以内の整数を選んで総和が A になるようにすることが可能か判定せよ。
 * 可能ならば "YES" と出力し、不可能ならば "NO" と出力せよ。
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
  int n, M, K;
  cin >> n >> M >> K;

  vector<vector<int>> dp(n + 1, vector<int>(M + 10, -1));

  dp[0][0] = 0; // 忘れない

  int as[M + 1];
  for (int i = 0; i < n; ++i) {
    cin >> as[i];
  }

  for (int i = 0; i < n; i++) {
    for (int m = 0; m <= M; m++) {
      // いったん前の状態を無条件に引き継ぐため条件の中に入れない
      dp[i + 1][m] = dp[i][m];

      if (as[i] <= m) {
        if (dp[i][m - as[i]] == -1) {
          dp[i + 1][m] = K;
        }
        else {
          dp[i + 1][m] = dp[i][m - as[i]] - 1;
        }
      }
    }
  }

  // debug
  for (int i = 0; i <= n; i++) {
    for (int m = 0; m <= M + 1; m++) {
      cout << std::right << std::setw(10) << dp[i][m];
    }
    cout << endl;
  }

  // cout << (dp.at(n).at(M) == -1 ? "No" : "Yes") << endl;
}