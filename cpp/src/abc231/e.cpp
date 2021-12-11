/*
Minimal payments
*/
#include <iostream>
#include <map>
#include <vector>
#define lps(i, j, n) for (int i = j; i < n; i++)

using namespace std;
static const int MMAX = 60; // コインの数
// static const int NMAX = 1 << 18; // コインの最大値
static const int INF = 1 << 29;

auto main() -> int
{
  int N, X;
  cin >> N >> X;
  vector<int> c(MMAX + 1), t(X + 1);
  for (int i = 1; i <= N; i++) // 1-indeded
    cin >> c[i];

  int ans = INF;
  for (int i = 0; i < N; i++) {
    int m = X; // あまり
    for (int j = N - 1 - i; j >= 0; j--) {
      while (m - c[j] >= 0) {
        cout << m << endl;
        m -= c[j];
      }
    }
    return 0;

    // あまりをコイン問題
    t.assign(X + 1, INF);

    t[0] = 0;

    for (int x : c)
      for (int j = 0; j + x <= m; j++)
        t[j + x] = min(t[j + x], t[j] + 1);
    ans = min(t[m], ans);
  }

  // cout << ans << endl;
}
