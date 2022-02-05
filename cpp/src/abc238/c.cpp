/* @cpg_dirspec c
digitnum

TLE ver
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto f(ll num) -> ll
{
  string tmp;

  tmp = to_string(num);

  // cout << tmp.size() << endl;
  // cout << num << endl;
  if (tmp.size() <= 1) {
    // cout << "in" << endl;
    return num;
  }

  string s = "1";
  while (s.length() < tmp.size()) {
    s += "0";
  }

  ll min = std::stoi(s);

  // cout << min << endl;
  // cout << num << endl;

  // exit(0);
  return (num - min + 1);
}

auto main() -> int
{
  int N;
  cin >> N;

  ll ans = 0;
  for (int i = 1; i <= N; i++) {
    ans += f(i);
  }

  cout << (ans % 998244353) << endl;
}