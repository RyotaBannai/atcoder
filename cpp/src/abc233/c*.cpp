/*
Product

袋数（for loop の対象）が n で決まっていない時は、dfs を使って i-th を順に試す.
*/
#include <functional>
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

ll ans = 0;
ll n, x;
vector<vector<ll>> a;
void dfs(ll pos, ll seki)
{
  if (pos == n) {
    if (seki == x)
      ans++;
    return;
  }
  for (ll c : a[pos]) {
    if (seki > x / c)
      // 次の自然数をかける前に求めたい x を seki が超えている時は見ても仕方ないから pass
      continue;
    dfs(pos + 1, seki * c);
  }
}

auto main() -> int
{
  cin >> n >> x;
  a.resize(n);
  for (ll i = 0; i <= n - 1; i++) {
    ll L;
    cin >> L;
    a[i].resize(L);
    for (ll j = 0; j <= L - 1; j++)
      cin >> a[i][j];
  }
  dfs(0, 1);
  cout << ans << endl;
  return 0;
}
