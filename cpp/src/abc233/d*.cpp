/*
Count Interval
*/
#include <iostream>
#include <map>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll n, k;
  cin >> n >> k;
  vector<ll> a(n);
  for (int i = 0; i < n; i++)
    cin >> a[i];

  // 累積和
  vector<ll> s(n + 1);
  for (int i = 0; i < n; i++)
    s[i + 1] = s[i] + a[i];

  // debug
  // for (auto x : s)
  //   cout << x << " ";
  // cout << endl;

  map<ll, ll> mp;
  ll ans = 0;
  for (int r = 1; r <= n; r++) {
    // ある地点から k だけ増加したかどうかをみたいため、start 地点の累積和を追加
    ll srt_sum = s[r - 1];
    // r 地点で増減した累積和から k を引いた時に、srt_sum と同じかどうかチェック
    // =srt_sum~s[r] での増加は k
    ll rest = s[r] - k;
    // map に追加するのは累積和の srt_base のみ.
    // 複数回同じ base になる可能性があり、その度に +1 して n 回区間を作れることを考慮.
    mp[srt_sum]++;
    ans += mp[rest];

    // debug
    // cout << "s[r - 1] " << s[r - 1] << endl;
    // cout << "s[r] - k " << rest << endl;
    // cout << "mp[" << rest << "] " << mp[rest] << endl;
  }
  cout << ans << endl;
  return 0;
}
