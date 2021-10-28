/*
与えられた N x M の庭にある水溜りの数を数える:
・一つの水溜りとみなすとき、庭のある点 i, j の 8 近傍で別の点と隣接している
・N x M 回の計算量になるが、水溜りになっていなければ continue 、また
・一回ごとに i, j の水溜りから戻す処理をする
*/
#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;

int n, m; // n := 整数の数, m := 合計の数
vector<int> v;

auto dfs(int i, int sum) -> bool {}

auto main() -> int
{
  int cnt;
  cin >> n >> m;
  v.reserve(n);
  lp(i, n) cin >> v[i];

  lp(i, n) lp(j, m)
  {
    dfs(i, j);
    cnt++;
  }
  cout << cnt << endl;
}