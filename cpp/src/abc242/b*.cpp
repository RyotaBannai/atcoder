/* @cpg_dirspec b
Minimize Ordering

https://atcoder.jp/contests/abc242/tasks/abc242_b

 */
#include <iostream>
#include <set>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  string str;
  cin >> str;

  multiset<char> S;
  for (auto c : str) {
    S.insert(c);
  }

  for (auto c : S) {
    cout << c;
  }
  cout << endl;
}