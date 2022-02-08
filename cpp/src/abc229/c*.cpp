/* @cpg_dirspec c
Cheese
*/
#include <iostream>
#include <queue>
using namespace std;
using ll = long long;

auto main() -> int
{
  priority_queue<pair<ll, ll>> C;
  ll n, W;
  cin >> n >> W;
  for (int i = 0; i < n; i++) {
    int v, w;
    cin >> v >> w;
    C.emplace(v, w);
  }

  ll ans = 0, cnt = 0;
  while (!C.empty()) {
    pair<ll, ll> x = C.top();
    C.pop();
    if (x.second <= W - cnt) { // このチーズを全て使ってもまだ W に余がある
      ans += x.first * x.second;
      cnt += x.second;
    }
    else { // この残りの分しか今回のチーズ乗せられない
      ans += x.first * (W - cnt);
      break;
    }
  }

  cout << ans << endl;
}