/*
全ての蟻が竿から落ちる最小の時間と、最大の時間を計算せよ.
※ 最小の時間は、竿から最初の蟻が最初に落ちるまでの時間時間ではない
*/
#include <cmath>
#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;

static const int INF = 1 << 29;

int n, l; // n := 蟻の数, l := 竿の長さ
vector<int> a;
void solve()
{
  int maxv = -INF;
  int minv = -INF;
  lp(i, n) minv = max(minv, min(a[i], l - a[i]));
  lp(i, n) maxv = max(maxv, max(a[i], l - a[i]));
  cout << minv << " " << maxv << endl;
}

auto main() -> int
{
  cin >> n >> l;
  a.reserve(n);
  lp(i, n) cin >> a[i];
  solve();
}