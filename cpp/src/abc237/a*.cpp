/* @cpg_dirspec a
Not Overflow
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll big = ll(1) << 31;
  ll N;
  cin >> N;
  cout << (-big <= N && N < big ? "Yes" : "No") << endl;
}