/** @cpg_dirspec partial_sum_with_n_items
 *
 * K個以内部分和問題
 * https://algo-method.com/tasks/312
 *
 * n 個の正の整数 a[0],a[1],…,a[n−1] と正の整数 A が与えられる。
 * これらの整数から K 個以内の整数を選んで総和が A になるようにすることが可能か判定せよ。
 * 可能ならば "YES" と出力し、不可能ならば "NO" と出力せよ。
 *
 * 最小個数部分和問題 と同じ方法をとる
 * ある数値を作るときに作れるならその最小を、作れないなら HUGE が入っている DP に対して、
 * 　作れるのであれば、その時に K 以下で作れるかどうかを判定してあげればよい.
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
  int n, A, K;
  cin >> n >> A >> K;

  vector<vector<int>> dp(n + 1, vector<int>(A + 10, HUGE));

  dp[0][0] = 0; // 忘れない

  int as[A + 1];
  for (int i = 0; i < n; ++i) {
    cin >> as[i];
  }

  for (int i = 0; i < n; i++) {
    for (int a = 0; a <= A; a++) {
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

  cout << (dp.at(n).at(A) <= K ? "Yes" : "No") << endl;
}