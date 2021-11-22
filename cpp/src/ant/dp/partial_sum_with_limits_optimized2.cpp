/*
個数制限付き部分和問題最適化 NxK メモリを使った ver
in:
3 17
3 3
5 2
8 2

out:
Yes(3*3+8=17)
*/
#include <iostream>
#include <vector>
using namespace std;

static const int MAX_N = 100;
static const int MAX_K = 100000;
int dp[MAX_N + 1][MAX_K + 1];
int v[MAX_N], m[MAX_N]; // v:= 値, m:= 個数
int n, K;               // n:= 列の長さ, K:= 作りたい数

void solve()
{
  for (int i = 1; i <= n; i++) {
    for (int j = 0; j <= K; j++) {
      if (dp[i - 1][j] >= 0)
        dp[i][j] = m[i];
      else
        // 前の部分の条件がなくても seg fal にならない.
        dp[i][j] = (j >= v[i] && dp[i][j - v[i]] > 0) ? dp[i][j - v[i]] - 1 : -1;
    }
  }

  // for (int i = 0; i < n; i++) {
  //   for (int j = 0; j < K; j++) {
  //     cout << dp[i][j] << " ";
  //   }
  //   cout << endl;
  // }
  cout << (dp[n][K] >= 0 ? "Yes" : "No") << endl;
}

auto main() -> int
{
  cin >> n >> K;
  for (int i = 1; i <= n; i++)
    cin >> v[i], cin >> m[i];

  memset(dp, -1, sizeof(dp));
  dp[0][0] = 0;

  solve();
}