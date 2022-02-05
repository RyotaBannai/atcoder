/* @cpg_dirspec e
Range Sums
 */
#include <iostream>
#include <set>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N, Q;
  cin >> N >> Q;
  set<int> s;
  for (int i = 1; i <= N; i++) {
    s.insert(i);
  }

  int b, e;
  while (Q--) {
    cin >> b >> e;
    auto bi = s.find(b);
    auto ei = s.find(e);

    if (bi != s.end() && ei != s.end()) { // どっちも見つかる場合
      s.erase(bi, next(ei, 1));
    }
    else if (bi != s.end() && ei == s.end()) { // 上が削除済み
      s.erase(bi, s.lower_bound(e));
    }
    else if (bi == s.end() && ei != s.end()) {
      s.erase(s.upper_bound(b), next(ei, 1));
    }
  }

  // for (auto x : s) {
  //   cout << x << endl;
  // }
  cout << (s.find(1) == s.end() && s.find(N) == s.end() ? "Yes" : "No") << endl;
}