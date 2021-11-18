/*
in:
4 5
2 3
1 2
3 4
2 2

out:
7
*/
#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;

static const int MAX_N = 100;
int w[MAX_N], v[MAX_N];
int n, W;

// i-th product, j := current remained weight.
auto rec(int i, int j) -> int
{
  int res;
  if (i == n) {
    // 品物は残っていない
    res = 0;
  }
  else if (j < w[i]) {
    res = rec(i + 1, j);
  }
  else {
    res = max(rec(i + 1, j), rec(i + 1, j - w[i]) + v[i]);
  }
  return res;
}

auto main() -> int
{
  cin >> n >> W;
  lp(i, n) cin >> w[i], cin >> v[i];

  cout << rec(0, W) << endl;
}