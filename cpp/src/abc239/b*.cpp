/* @cpg_dirspec b
 */
#include <cmath>
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll x;
  cin >> x;

  if (x < 0 && x % 10 != 0) { // 少数部分のチェックには剰余を用いればいい...
    cout << (x / 10) - 1 << endl;
  }
  else {
    cout << (x / 10) << endl;
  }
}