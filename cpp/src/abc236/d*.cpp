/* @cpg_dirspec d
 */
#include <iostream>
#include <map>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

int a[20][20];
bool used[20];
vector<pair<int, int>> vec;
int n;

auto calc() -> int
{
  if (vec.size() == n) {
    int ret = 0;
    for (auto x : vec) {
      ret ^= a[x.first][x.second];
    }
    return ret;
  }

  int l;
  for (int i = 1; i <= 2 * n; i++) {
    if (!used[i]) {
      l = i;
      break;
    }
  }
  used[l] = true;

  int ret = 0;
  for (int i = 1; i <= 2 * n; i++) {
    if (!used[i]) {
      vec.emplace_back(l, i), used[i] = true;
      ret = max(ret, calc());
      vec.pop_back(), used[i] = false;
    }
  }

  used[l] = false;
  return ret;
}

auto main() -> int
{
  cin >> n;

  // 組合せの入力を 2 次元配列で保持
  for (int i = 1; i <= n * 2 - 1; i++) {
    for (int j = i + 1; j <= n * 2; j++) {
      cin >> a[i][j];
    }
  }

  cout << calc() << endl;
}