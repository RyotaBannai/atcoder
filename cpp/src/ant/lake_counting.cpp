/*
与えられた N x M の庭にある水溜りの数を数える:
・一つの水溜りとみなすとき、庭のある点 i, j の 8 近傍で別の点と隣接している
・N x M 回の計算量になるが、水溜りになっていなければ continue 、また
・一回ごとに i, j の水溜りから戻す処理をする

10 12
W........WW.
.WWW.....WWW
....WW...WW.
.........WW.
.........W..
..W......W..
.W.W.....WW.
W.W.W.....W.
.W.W......W.
..W.......W.

3

*/
#include <iostream>
#include <string>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;
const static char POND = 'W';
const static char SAND = '.';

int n, m; // n := 庭の行数, m := 庭の列数
vector<vector<int>> v;

void dfs(int i, int j)
{
  if (v[i][j] == SAND)
    return;

  // 水溜りを SAND で埋める
  v[i][j] = SAND;

  // 8 近傍を探索
  if (j - 1 >= 0) {
    dfs(i, j - 1); // 左
    if (i - 1 >= 0)
      dfs(i - 1, j - 1); // 左上
    if (i + 1 < n)
      dfs(i + 1, j - 1); // 左下
  }

  if (j + 1 < m) {
    dfs(i, j + 1); // 右
    if (i - 1 >= 0)
      dfs(i - 1, j + 1); // 右上
    if (i + 1 < n)
      dfs(i + 1, j + 1); // 右下
  }

  if (i - 1 >= 0)
    dfs(i - 1, j); // 上
  if (i + 1 < n)
    dfs(i + 1, j); // 下

  return;
}

void dfs2(int i, int j)
// dfs と同じ
{
  v[i][j] = SAND;

  // 8 近傍を探索
  // -1<=i<=1, -1<=j<=1 の 8 通り(i==0, j==0 の場合以外)
  lps(k, -1, 2) lps(l, -1, 2)
  {
    int next_i = i + k;
    int next_j = j + l;
    if (next_i >= 0 && next_i < n && next_j >= 0 && next_j < m && v[next_i][next_j] == POND)
      dfs(next_i, next_j);
  }

  return;
}

auto main() -> int
{
  int cnt = 0;
  string line;
  cin >> n >> m;
  v.reserve(n);
  lp(i, n)
  {
    cin >> line, v[i].reserve(m);
    lp(j, m) v[i][j] = line[j];
  };

  lp(i, n) lp(j, m)
  {
    if (v[i][j] == SAND)
      continue;
    dfs(i, j);
    cnt++;
  }
  cout << cnt << endl;
}