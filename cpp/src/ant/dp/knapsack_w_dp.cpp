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
// i x j の mat
// i 番目以降の品物から重さの総和が j 以下となるように選んだ時の、価値の総和の最大値
int dp[MAX_N][MAX_N];
int w[MAX_N], v[MAX_N];
int n, W;

auto rec(int i, int j) -> int
{
  // すでに調べたことがあるならばその結果を再利用. p53
  if (dp[i][j] >= 0)
    return dp[i][j];

  int res;
  if (i == n) {
    res = 0;
  }
  else if (j < w[i]) {
    res = rec(i + 1, j);
  }
  else {
    res = max(rec(i + 1, j), rec(i + 1, j - w[i]) + v[i]);
  }
  return dp[i][j] = res; // 結果をテーブルに記憶する
}

auto main() -> int
{
  cin >> n >> W;
  lp(i, n) cin >> w[i], cin >> v[i];

  // まだ調べていないことを表す -1 でメモ化テーブルを初期化
  // memset(void* dest, int ch, std::size_t count);
  memset(dp, -1, sizeof(dp)); // memset(dp, -1, sizeof dp);

  cout << rec(0, W) << endl;
}