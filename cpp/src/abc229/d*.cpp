/* @cpg_dirspec d
Longest X

. の累積和を利用
X をどこに埋めるかではなく、. が許容値 k を超えない範囲でチェック範囲を伸縮させながらテスト
→ 累積和
*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  string s;
  ll k;
  cin >> s >> k;
  ll n = s.size();
  vector<ll> cnt(n + 1); // 累積和
  // cnt[r] - cnt[l] で s[l,r) の '.' の数を表す
  for (ll i = 0; i < n; i++) {
    if (s[i] == '.')
      cnt[i + 1] = cnt[i] + 1;
    else
      cnt[i + 1] = cnt[i];
  }

  // debug
  // for (auto x : cnt)
  //   cout << " " << x;
  // cout << endl;

  ll ans = 0;
  ll r = 0;
  for (ll l = 0; l < n; l++) {
    // 累積和の [l,r) を使う. 次の要素との差分をテスト
    // r+1 で先に超えないかチェックしてから increment
    while (r < n && cnt[r + 1] - cnt[l] <= k) {
      r++;
    }
    // a=XX...X.X.X., s=2 のとき r=4 でbreak
    ans = max(ans, r - l);
  }
  cout << ans << endl;
}