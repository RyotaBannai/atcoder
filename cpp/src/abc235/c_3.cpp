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
    A.emplace(a, i + 1);
  }

  while (Q--) {
    int x, k;
    cin >> x >> k;
    auto ret = A.equal_range(x); // lower_bound, upper_bound
    if (ret.first == A.end()) {
      cout << (-1) << endl;
      continue;
    }

    if (k <= distance(ret.first, ret.second)) {
      advance(ret.first, k - 1);
      cout << ret.first->second << endl;
    }
    else
      cout << (-1) << endl;
  }
}