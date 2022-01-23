/* @cpg_dirspec b
 */
#include <iostream>
#include <map>
#include <string>
using namespace std;
using ll = long long;

auto main() -> int
{
  int n;
  cin >> n;
  map<int, int> m;
  for (int i = 0; i < n * 4 - 1; i++) {
    int a;
    cin >> a;
    m[a]++;
  }

  int ans = 0;
  for (auto x : m) {
    if (x.second == 3) {
      cout << x.first << endl;
      exit(0);
    }
  }
}