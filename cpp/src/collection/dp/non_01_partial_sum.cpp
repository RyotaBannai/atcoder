/**
 * non 01 部分和問題（個数制限付き部分和問題）
 * https://algo-method.com/tasks/313
 *
 * N 種類の正の整数 A0,A1,⋯,AN−1 と正の整数 M が与えられます。
 * いくつかを選んで総和が M となるようにできるか判定してください。
 * ただし、整数 Ai は Bi 個まで選ぶことができます。 (0≤i≤N−1)
 *
 *
 * 方針：
 * N 商品を Sn 個使って数値 M にできるかどうか判定
 * Sn 個使えるため、Sn0~Snk の間で k 個使用した時にその差分魔まで遡って True/False 判定を行う
 * k 個以上使わない、k*数が遡る数値を超えないように制御（マイナス index にしない）
 *
 * 最大価値を試す処理について
 * https://github.com/RyotaBannai/atcoder/blob/master/cpp/src/ant/dp/knapsack/multiple_objects_knapsack.cpp
 */

#include <iomanip> // std::setw(int), std::setfill(char)
#include <ios>     // std::left, std::right
#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;

auto main() -> int
{
  // 入力
  int N, M;
  cin >> N >> M;

  vector<vector<bool>> dp(N + 10, vector<bool>(M + 10, false));
  dp.at(0).at(0) = true; // 忘れない

  int as[N + 1], ac[N + 1];
  for (int i = 0; i < N; ++i) {
    cin >> as[i] >> ac[i];
  }

  for (int i = 0; i < N; ++i) {
    for (int j = 0; j <= M; ++j) {
      for (int k = 0; k <= ac[i] && k * as[i] <= j; ++k) {
        dp[i + 1][j] = dp[i][j] || dp[i][j - k * as[i]];
      }
    }
  }

  // debug
  // for (int i = 0; i <= N; i++) {
  //   for (int a = 0; a <= M + 1; a++) {
  //     cout << std::right << std::setw(5) << dp[i][a];
  //   }
  //   cout << endl;
  // }

  cout << (dp[N][M] ? "Yes" : "No") << endl;
}