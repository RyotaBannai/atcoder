/*
input:

4
1 1048577
1 1
2 2097153
2 3

output:
1048577
-1
*/
#include <iostream>
#include <set>
#define ll long long
using namespace std;

const int N = 1 << 20;
int n, m, k;
set<int> s;
ll x[N];

auto main() -> int
{
  int T;
  cin >> T;
  for (int i = 0; i < N; ++i)
    s.insert(i), x[i] = -1;
  while (T--) {
    int op;
    ll v;
    cin >> op >> v;
    if (op == 1) {
      auto pos = s.lower_bound(v % N);
      // set の v % N の位置を削除した結果、
      // 以後の回で v % N 以下の位置が見つからない場合
      // lower_bound() の結果が s.end() になる.
      // その際は h++ をして次の最小を見つけるように
      // 最も小さい数(ここでは 0)から次の位置を見るけるようにする.
      if (pos == s.end()) {
        pos = s.lower_bound(0);
        // 一度見つけたら値を入れ、set から削除.
        x[*pos] = v;
        s.erase(*pos);
      }
      else {
        x[*pos] = v;
        s.erase(*pos);
      }
    }
    else {
      cout << x[v % N] << endl;
    }
  }
  return 0;
}
