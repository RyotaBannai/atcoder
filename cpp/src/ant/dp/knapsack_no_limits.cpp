/*
ナップザック問題, 同じ品物を複数選択可能なパターン
in:
3 7
3 4
4 5
2 3

out:
10
*/
#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;

static const int MAX_N = 100;
int dp[MAX_N][MAX_N];
int w[MAX_N], v[MAX_N];
int n, W;

void solve(int i, int j)
{
  for (int i = 0; i < n; i++) {
    for (int j = 0; j <= W; j++) {
      /*
      j(:= 許容する重さ)を制限容量(:= W)まで１つずつ増やしていき、
      0 <= j <= W のレンジ内で複数使用できる品物を W を超えないようにループしながら
      使用する個数を１つずつ増やして最大価値になるかどうか試す
      */
      for (int k = 0; k * w[i] <= j; k++) {
        dp[i + 1][j] = max(dp[i + 1][j], dp[i][j - k * w[i]] + k * v[i]);
      }
    }
  }
  cout << dp[n][W] << endl;
}

auto main() -> int
{
  cin >> n >> W;
  lp(i, n) cin >> w[i], cin >> v[i];

  memset(dp, 0, sizeof(dp));

  solve(0, W);
}