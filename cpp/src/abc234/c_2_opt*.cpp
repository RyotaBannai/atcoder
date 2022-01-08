/* @cpg_dirspec c
Happy New Year!
*/
#include <iostream>
#include <string>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll K;
  cin >> K;
  string ans = "";
  while (true) {
    ans += (K % 2) ? "2" : "0";
    K /= 2;
    if (!K)
      break;
  }
  reverse(ans.begin(), ans.end());
  cout << ans << endl;
}