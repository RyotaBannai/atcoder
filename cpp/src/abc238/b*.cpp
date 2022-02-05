/* @cpg_dirspec b
Pizza
 */
#include <iostream>
#include <set>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N;
  cin >> N;
  set<int> s;
  for (int i = 1; i <= 360; i++) {
    s.insert(i);
  }

  int total = 0;
  for (int i = 0; i < N; i++) {
    int rad;
    cin >> rad;
    total += rad;
    s.erase(total % 360);
  }

  // for (int i = 1; i < 360; i++) {
  //   if (s.find(i) == s.end()) { // erase = 切れ目
  //     cout << i << endl;
  //   }
  // }

  int ans = 0, prev = 0;
  for (int i = 1; i <= 360; i++) {
    ans = max(ans, i - prev);
    if (s.find(i) == s.end()) { // erase = 切れ目
      prev = i;
    }
  }

  cout << ans << endl;
}