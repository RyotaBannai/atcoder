/* @cpg_dirspec b
Climbing Takahashi
*/
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N;
  cin >> N;
  int ans = 0, nex;
  while (N--) {
    cin >> nex;
    if (ans < nex)
      ans = nex;
    else
      break;
  }
  cout << ans << endl;
}