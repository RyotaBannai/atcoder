/* @cpg_dirspec c
 */
#include <cmath>
#include <iomanip> // for std::setprecision()
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;
static const float EPSILON = 0.0001;

bool AreSame(double a, double b) { return fabs(a - b) < EPSILON; }

auto main() -> int
{
  ll a, b, c, d;
  cin >> a >> b >> c >> d;
  pair<ll, ll> one(a + 2, b + 1), two(a + 1, b + 2), three(a - 2, b - 1), four(a - 1, b - 2);

  auto len1 = std::sqrtf((c - one.first) * (c - one.first) + (d - one.second) * (d - one.second));
  auto len2 = std::sqrtf((c - two.first) * (c - two.first) + (d - two.second) * (d - two.second));
  auto len3 =
      std::sqrtf((c - three.first) * (c - three.first) + (d - three.second) * (d - three.second));
  auto len4 =
      std::sqrtf((c - four.first) * (c - four.first) + (d - four.second) * (d - four.second));

  cout << setprecision(12) << len1 << endl;
  cout << len2 << endl;
  // cout << len3 << endl;
  // cout << len4 << endl;
  cout << ((AreSame(len1, 2.23607) || AreSame(len2, 2.23607) || AreSame(len3, 2.23607) ||
            AreSame(len4, 2.23607))
               ? "Yes"
               : "No");
}