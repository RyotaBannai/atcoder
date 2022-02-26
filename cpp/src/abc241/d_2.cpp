/* @cpg_dirspec d

 */
#include <algorithm>
#include <iostream>
#include <map>
#include <string>
#include <vector>
using namespace std;
using ll = long long;
using M = multimap<int, int>;

auto main() -> int
{
  int N;
  M m;
  cin >> N;
  int x, k;

  while (N--) {
    int q;
    cin >> q;
    if (q == 1) {
      cin >> x;
      m.emplace(x, x);
    }
    else if (q == 2) {
      cin >> x >> k;
      auto it = m.upper_bound(x); // x より小さい

      if (distance(m.begin(), it) < k) {
        cout << -1 << endl;
      }
      else {
        advance(it, -k);
        cout << it->first << endl;
      }
    }
    else if (q == 3) {
      cin >> x >> k;
      auto it = m.lower_bound(x); // x より大きい
      if (distance(it, m.end()) < k) {
        cout << -1 << endl;
      }
      else {
        advance(it, k - 1);
        cout << it->first << endl;
      }
    }
  }
}