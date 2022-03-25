/* @cpg_dirspec d
Play Train

*/
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll n, q;
  cin >> n >> q;
  ll nil = -1;
  vector<ll> front(n + 1, nil), back(n + 1, nil);
  while (q--) {
    ll c, x, y;
    cin >> c;
    if (c == 3) {
      cin >> x;
      while (front[x] != nil)
        x = front[x]; // 先頭の front は nil // while を抜ける時点で先頭の x == inx

      vector<ll> ans;
      while (x != nil)
        ans.push_back(x), x = back[x]; // while を抜けた時は x == nil

      string s;
      ll cnt = 0;
      for (auto z : ans) {
        if (cnt++ > 0)
          s += " ";
        s += to_string(z);
      }
      cout << cnt << " " << s << endl;
    }
    else {
      cin >> x >> y;
      if (c == 1)
        back[x] = y, front[y] = x;
      else if (c == 2)
        back[x] = nil, front[y] = nil;
    }
  }
}