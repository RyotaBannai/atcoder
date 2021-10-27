/*
与えられた n 個の数字を任意の個数使い、目的の合計値 m を作れるかどうかの判定.
4 13
1 2 4 7

4 15
1 2 4 7
*/
#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;

int n, m; // n := 整数の数, m := 合計の数
vector<int> v;

auto dfs(int i, int sum) -> bool
{
  // i + 1 で dfs に入ると n + 1 の状態になるがこの一個多い状態の先頭で判定を行うため問題ない
  if (i == n)
    return sum == m;

  // i 番目の要素を加えない場合
  if (dfs(i + 1, sum))
    return true;
  // i 番目の要素を加える場合
  if (dfs(i + 1, sum + v[i]))
    return true;

  return false;
}

auto main() -> int
{
  cin >> n >> m;
  v.reserve(n);
  lp(i, n) cin >> v[i];
  cout << (dfs(0, 0) ? "Yes" : "No") << endl;
}