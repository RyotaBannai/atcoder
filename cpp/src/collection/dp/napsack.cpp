/**
 * ナップサック問題
 * https://algo-method.com/tasks/308
 *
 */
#include <iostream>
using namespace std;

auto main() -> int
{
  // 入力
  int n, W;
  cin >> n >> W;

  int weight[n + 1], value[n + 1]; // weight, value
  int dp[n + 1][W + 1];            // DP テーブル // weight に対する dp を保持
  memset(dp, 0, sizeof(dp));
  // DP初期条件: dp[0][w] = 0

  for (int i = 0; i < n; ++i) {
    cin >> weight[i] >> value[i];
  }

  for (int i = 0; i < n; i++) {
    for (int w = 0; w <= W; w++) {
      if (weight[i] > w) { // ナップサック w が商品より小さい時は、前回分をそのまま使用
        dp[i + 1][w] = dp[i][w];
      }
      else {
        // 後半部分は一つ前の回から引くことで品物を高々一つだけ選択できるように設計できる
        dp[i + 1][w] = max(dp[i][w - weight[i]] + value[i], dp[i][w]);
      }
    }
  }

  cout << dp[n][W] << endl;
}