/* @cpg_dirspec a
Weird Function
*/
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto f(ll x) -> ll { return x * x + 2 * x + 3; }

auto main() -> int
{
  ll t;
  cin >> t;
  // f(f(f(t)+t)+f(f(t)))
  ll ans = f(f(f(t) + t) + f(f(t)));
  cout << ans << endl;
}