/*
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
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;

static const int MAX_N = 100;
int dp[MAX_N][MAX_N];
int w[MAX_N], v[MAX_N];
int n, W;

void solve(int i, int j)
{
  for (int i = 0; i < n; i++) {
    for (int j = 0; j <= W; j++) {
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