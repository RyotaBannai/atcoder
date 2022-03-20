/* @cpg_dirspec b
Go Straight and Turn Right

https://atcoder.jp/contests/abc244/tasks/abc244_b

 */
#include <iomanip> // for std::setprecision()
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N;
  string s;
  cin >> N >> s;
  vector<pair<int, int>> vec{{1, 0}, {0, -1}, {-1, 0}, {0, 1}}; // 東、南、西、北

  int d = 0;
  int i = 1;
  pair<int, int> pos = {0, 0};
  for (auto c : s) {
    if (c == 'R') {
      d++;
      d %= 4;
      // cout << d << endl;
    }
    else if (c == 'S') {
      pos.first += vec[d].first;
      pos.second += vec[d].second;
    }
    // cout << i++ << ": " << pos.first << " " << pos.second << endl;
  }

  cout << pos.first << " " << pos.second << endl;
}