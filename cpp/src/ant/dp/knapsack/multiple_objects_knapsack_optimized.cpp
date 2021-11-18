/*
ナップザック問題, 同じ品物を複数選択可能なパターン, k のループ分の計算量を削減
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
      ・品物の重さが j より小さい時は使えないため、i までの最大をコピーし、それ以外の場合は
      ・i までの品物を複数回使った場合の最大を、今回分で最大を更新できるか試す

      ポイント：
      ・ i+1 の品物を複数回使うケースでは、一つ前の j - w[i] を i+1 の品物で更新できるかどうかは
      すで試しているため、今回の j + w[i] では j + w[i] - w[i] を考慮した最大となる
      */
      if (j < w[i])
        dp[i + 1][j] = dp[i][j];
      else
        dp[i + 1][j] = max(dp[i][j], dp[i + 1][j - w[i]] + v[i]);
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