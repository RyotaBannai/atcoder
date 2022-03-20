/* @cpg_dirspec d
Swap Hats

https://atcoder.jp/contests/abc244/tasks/abc244_d

 */
#include <iomanip> // for std::setprecision()
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  string s, t;
  int n = 3;
  for (int i = 0; i < n; i++) {
    char c;
    cin >> c;
    s += c;
  }

  for (int i = 0; i < n; i++) {
    char c;
    cin >> c;
    t += c;
  }

  vector<string> e = {"RGB", "GBR", "BRG"};
  vector<string> o = {"GRB", "RBG", "BGR"};

  auto it = std::find(e.begin(), e.end(), s);
  if (it != e.end()) {
    auto itt = std::find(e.begin(), e.end(), t);
    cout << (itt != e.end() ? "Yes" : "No") << endl;
  }
  else {
    auto itt = std::find(o.begin(), o.end(), t);
    cout << (itt != o.end() ? "Yes" : "No") << endl;
  }
}