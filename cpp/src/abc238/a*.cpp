/* @cpg_dirspec a
Exponential or Quadratic
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll n;
  cin >> n;
  if (n > 64) {
    cout << "Yes" << endl;
    exit(0);
  }
  cout << ((1 << n) > (n * n) ? "Yes" : "No") << endl;
}