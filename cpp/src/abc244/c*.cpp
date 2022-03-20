/* @cpg_dirspec c
Yamanote Line Game

https://atcoder.jp/contests/abc244/tasks/abc244_c

 */
#include <iomanip> // for std::setprecision()
#include <iostream>
#include <set>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N;
  cin >> N;
  set<int> s;
  int max = 2 * N + 1;

  int a;

  for (int i = 1; i <= max; ++i) {
    s.insert(i);
  }
  while (true) {
    // std::cin.ignore(INT_MAX);

    auto x = s.lower_bound(1);
    s.erase(*x);
    cout << *x << endl;

    cin >> a;
    if (a == 0) {
      exit(0);
    }
    else {
      s.erase(a);
    }
  }
}