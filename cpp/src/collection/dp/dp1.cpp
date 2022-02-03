/**
 * 最大和問題
 * https://algo-method.com/tasks/307
 *
 * n 個の整数 a[0],a[1],…,a[n−1] が与えられる。
 * これらの整数から何個かの整数を選んで総和をとったときの、総和の最大値を求めよ。
 * また、何も選ばない場合の総和は 0 であるものとする。
 */
#include <algorithm>
#include <iostream>
using namespace std;

// 入力
int n;
int a[10010];

// DP テーブル
int dp[10010];

auto main() -> int
{
  cin >> n;
  for (int i = 0; i < n; ++i)
    cin >> a[i];

  dp[0] = 0;
  for (int i = 0; i < n; ++i) {
    dp[i + 1] = max(dp[i], dp[i] + a[i]);
  }

  cout << dp[n] << endl;
}