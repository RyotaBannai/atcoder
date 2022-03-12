/* @cpg_dirspec a
Shampoo

https://atcoder.jp/contests/abc243/tasks/abc243_a

 */
#include <iomanip> // for std::setprecision()
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int v, a, b, c;
  cin >> v >> a >> b >> c;

  int res = v;
  while (true) {
    res -= a;
    if (res < 0) {
      cout << "F" << endl;
      exit(0);
    }

    res -= b;
    if (res < 0) {
      cout << "M" << endl;
      exit(0);
    }

    res -= c;
    if (res < 0) {
      cout << "T" << endl;
      exit(0);
    }
  }
}