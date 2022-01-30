/* @cpg_dirspec d
LR insertion
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N;
  cin >> N;
  string s;
  cin >> s;
  vector<int> V{0};
  for (int i = 1; i <= N; i++) {
    if (s[i - 1] == 'L') {
      V.insert(V.begin() + i - 1, i);
    }
    if (s[i - 1] == 'R') {
      V.insert(V.begin() + i, i);
    }
  }

  for (auto x : V) {
    cout << x << " ";
  }
  cout << endl;
}