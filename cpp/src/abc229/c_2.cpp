/*
Cheese

TLE ver.
*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  vector<pair<int, int>> C;
  ll n, W;
  cin >> n >> W;
  for (int i = 0; i < n; i++) {
    int v, w;
    cin >> v >> w;
    C.emplace_back(v, w);
  }

  sort(C.begin(), C.end(), greater<>());

  // for (auto &x : C) {
  //   cout << x.first << " " << x.second << endl;
  // }
  ll ans = 0;
  while (W--) {
    for (auto &x : C) {
      if (x.second != 0) {
        ans += x.first;
        --x.second;
        break;
      }
    }
  }

  cout << ans << endl;
}