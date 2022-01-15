/* @cpg_dirspec c
 */
#include <iostream>

#include <map>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N, Q;
  cin >> N >> Q;
  multimap<int, int, std::less<>> A;
  for (int i = 0; i < N; i++) {
    int a;
    cin >> a;
    // cout << a << endl;
    A.emplace(a, i);
  }

  // cout << A.count(1) << endl;
  // for (auto x : A) {
  //   cout << x.first << ":" << x.second << endl;
  // }

  while (Q--) {
    // cout << endl << "Q " << Q << endl;
    int x, k;
    cin >> x >> k;
    auto low = A.lower_bound(x);
    auto up = A.upper_bound(x);
    if (low == A.end()) // not fond
    {
      cout << (-1) << endl;
      continue;
    }

    bool found = false;
    for (auto i = low; i != up; i++) {
      k--;
      // cout << "k: " << k << endl;
      if (!k) {
        cout << (i->second + 1) << endl;
        found = true;
        break;
      }
    }
    if (!found)
      cout << (-1) << endl;
  }
}