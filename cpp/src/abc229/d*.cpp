/*
Longest X

Official ver.
*/

#include <iostream>
#include <vector>
using namespace std;
using ll = long long;
auto main() -> int
{
  string s;
  cin >> s;
  ll k;
  cin >> k;
  ll n = s.size();
  vector<ll> cnt(n + 1); // 累積和
  for (ll i = 0; i < n; i++) {
    if (s[i] == '.')
      cnt[i + 1] = cnt[i] + 1;
    else
      cnt[i + 1] = cnt[i];
  } // cnt[r] - cnt[l] で s[l,r) の '.' の数を表す

  // debug
  // for (auto x : cnt)
  //   cout << " " << x;
  // cout << endl;

  ll ans = 0;
  ll r = 0;
  for (ll l = 0; l < n; l++) {
    while (r < n && cnt[r + 1] - cnt[l] <= k) { // 累積和の [l,r) を使う. 次の要素との差分をテスト
      r++;
    }
    // a=XX...X.X.X., s=2 のとき r=4 でbreak
    ans = max(ans, r - l);
  }
  cout << ans << endl;
}