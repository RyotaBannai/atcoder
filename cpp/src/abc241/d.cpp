/* @cpg_dirspec d

TLE
 */
#include <cmath>
#include <iostream>
#include <set>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll N;
  std::multiset<ll> s;
  cin >> N;

  while (N--) {
    int q;
    cin >> q;
    if (q == 1) {
      ll x;
      cin >> x;
      s.insert(x);
    }
    else if (q == 2) {
      ll x, k;
      cin >> x >> k;
      auto it = s.upper_bound(x); // x より小さい
      if (distance(s.begin(), it) < k) {
        cout << -1 << endl;
      }
      else {
        while (k--) {
          it--;
        }
        cout << *it << endl;
      }
    }
    else if (q == 3) {
      ll x, k;
      cin >> x >> k;
      auto it = s.lower_bound(x); // x より大きい
      if (distance(it, s.end()) < k) {
        cout << -1 << endl;
      }
      else {
        while (--k) {
          it++;
        }
        cout << *it << endl;
      }
    }
  }
}