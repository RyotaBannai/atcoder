/* @cpg_dirspec a
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int i, j;
  string s;
  cin >> s >> i >> j;
  for (int n = 0; n < s.length(); n++) {
    if (n == i - 1) {
      cout << s[j - 1];
    }
    else if (n == j - 1) {
      cout << s[i - 1];
    }
    else {
      cout << s[n];
    }
  }
  cout << endl;
}