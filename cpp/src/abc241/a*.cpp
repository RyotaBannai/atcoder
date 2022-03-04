/* @cpg_dirspec a
Digit Machine
https://atcoder.jp/contests/abc241/tasks/abc241_a

 */
#include <cmath>
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  vector<int> as;
  for (int i = 0; i <= 9; i++) {
    int x;
    cin >> x;
    as.push_back(x);
  }

  int ans = as[0];
  for (int i = 0; i < 2; i++) {
    ans = as[ans];
  }

  cout << ans << endl;
}