/* @cpg_dirspec b
 */
#include <cfenv>
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

  if (x < 0) {
    float integral_part = 0;
    ll y = std::floorl(x / (float)10);
    auto ff = modf(y, &integral_part);
    if (ff == -0) {
      cout << y << endl;
    }
    else {
      cout << (y - 1) << endl;
    }
  }
  else {
    ll y = std::floorl(x / 10);
    cout << y << endl;
  }
}