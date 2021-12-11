/*
 */
#include <iostream>
#include <vector>
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;

static const int MMAX = 20;    // コインの数
static const int NMAX = 50000; // 求めたい金額の最大値
static const int INF = 1 << 29;

// 普通のコイン問題と違って、額をオーバしてもよい.
// この方法だとオーバしないで最小を求めているので違う
auto main() -> int
{
  int n, m;
  vector<int> c(MMAX + 1), t(NMAX + 1);

  cin >> m >> n;
  lps(i, 1, m + 1) cin >> c[i];
  t.assign(NMAX + 1, INF);

  t[0] = 0;
  for (int x : c)
    for (int j = 0; j + x <= n; j++)
      t[j + x] = min(t[j + x], t[j] + 1);
  cout << t[n] << endl;

  return 0;
}