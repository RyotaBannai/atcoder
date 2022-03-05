/* @cpg_dirspec a

https://atcoder.jp/contests/abc242/tasks/abc242_a

 */
#include <iomanip> // for std::setprecision()
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  double A, B, C, X;
  cin >> A >> B >> C >> X;
  if (X <= A) {
    cout << 1.0 << endl;
  }
  else if (X > B) {
    cout << std::setprecision(16) << 0.0 << endl;
  }
  else {
    cout << std::setprecision(16) << double(C / (B - A)) << endl;
  }
}