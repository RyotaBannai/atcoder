/* @cpg_dirspec a
Rotate
*/
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int ans = 0, cnt = 3;
  char c, ci;
  while (cnt) {
    cnt--;
    cin >> c;
    ci = c - '0';
    ans += ci;
    ans += ci * 10;
    ans += ci * 100;
    // cout << ci << endl;
  }
  cout << ans << endl;
}