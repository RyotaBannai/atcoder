/*
ナップザック問題, 重さの制限が大きいパターンの最適化
in:
4 5
2 3
1 2
3 4
2 2

out:
7 (0,1,3 番の品物を選ぶ)
*/
#include <iostream>
#include <vector>
using namespace std;

static const int MAX_N = 100;
static const int MAX_V = 100;
static const int INF = 1 << 29;
// 品物の最大価値がどれくらいかは事前にわからないため、制約で指定されてる「最大価値 x 品物数」分確保
int dp[MAX_N + 1][MAX_N * MAX_V + 1];
int w[MAX_N], v[MAX_N];
int n, W;

void solve()
{
  for (int i = 0; i < n; i++) {
    for (int j = 0; j <= MAX_N * MAX_V; j++) {
      /*
      重さの最大が非常に大きいため、想定される価値の最大値でループを試す
      それぞれの element の値は重さを表すため、「特定の価値を出すために必要な最小の重さ」を考える
      */
      if (j < v[i])
        dp[i + 1][j] = dp[i][j];
      else
        dp[i + 1][j] = min(dp[i][j], dp[i][j - v[i]] + w[i]);
    }
  }
  int res = 0;
  for (int i = 0; i <= MAX_N * MAX_V; i++)
    if (dp[n][i] <= W) {
      // cout << "value:" << i << ", weight: " << dp[n][i] << endl;
      res = i;
    }

  cout << res << endl;
}

auto main() -> int
{
  cin >> n >> W;
  for (int i = 0; i < n; i++)
    cin >> w[i], cin >> v[i];

  fill(dp[0], dp[0] + MAX_N * MAX_V + 1, INF);
  dp[0][0] = 0;

  solve();
}