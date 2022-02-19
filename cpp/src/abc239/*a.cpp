/* @cpg_dirspec a
 */
#include <cmath>
#include <iomanip> // for std::setprecision()
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll H;
  cin >> H;
  cout << std::setprecision(16) << std::sqrtl((H * (12800000 + H))) << endl;
}