#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;
using ll = long long;

static const ll MAX_N = 3 * (10 << 5);
static const ll MAX_W = 3 * (10 << 8);
ll dp[MAX_N][MAX_W];
ll v[MAX_N];
int w[MAX_N];
ll n, W;

void solve()
{
  for (int i = 0; i < n; i++) {
    for (int j = 0; j <= W; j++) {
      cout << W << endl;
      if (w[i] > 0 && dp[i][j] < (dp[i + 1][j - 1] + v[i])) {
        dp[i + 1][j] = dp[i + 1][j - 1] + v[i];
        w[i]--;
      }
      else {
        dp[i + 1][j] = dp[i][j];
      }
    }
  }
  cout << dp[n][W] << endl;
}

auto main() -> int
{
  cin >> n >> W;
  for (int i = 0; i < n; i++)
    cin >> v[i] >> w[i];

  memset(dp, 0, sizeof(dp));

  solve();
}