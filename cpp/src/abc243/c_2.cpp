/* @cpg_dirspec c

https://atcoder.jp/contests/abc243/tasks/abc243_c


TLE ver.
 */
#include <algorithm>
#include <iomanip> // for std::setprecision()
#include <iostream>
#include <set>
#include <string>
#include <tuple>
#include <vector>

using namespace std;
using ll = long long;
using T = tuple<int, int, int>; // x,y,nth

auto main() -> int
{
  int N;
  cin >> N;
  vector<T> v;
  set<int> ys;

  for (int i = 0; i < N; ++i) {
    int x, y;
    cin >> x >> y;
    v.emplace_back(x, y, i);
    ys.insert(y);
  }

  string S;
  cin >> S;

  /*
  同じ y 座標同士にいるかどうか

  x 座標が交差するかどうか
  x1<x2 && S1 = 'R' && S2 = 'L'
  OR
  x1>x2 && S1 = 'L' && S2 = 'R'

  同じ y 座標同士を 組み合わせて上記を総当たりでチェック？
  取り出した item の 3rd で S の操作を取り出す？
  */

  for (int y : ys) {
    vector<T> sameys;
    std::copy_if(v.begin(), v.end(), std::back_inserter(sameys),
                 [y](T a) { return y == std::get<1>(a); });
    int len = sameys.size();

    auto m = minmax_element(v.begin(), v.end(),
                            [](const auto &a, const auto &b) { return get<0>(a) < get<0>(b); });

    if (get<2>(*m.first) == 'R' && get<2>(*m.second) == 'L') {
      cout << "Yes" << endl;
      exit(0);
    }

    for (int i = 0; i < len; ++i) {
      for (int j = i + 1; j < len; ++j) {
        int x1 = std::get<0>(sameys[i]);
        int x2 = std::get<0>(sameys[j]);
        char op1 = S[std::get<2>(sameys[i])];
        char op2 = S[std::get<2>(sameys[j])];
        if (x1 < x2 && op1 == 'R' && op2 == 'L') {
          cout << "Yes" << endl;
          exit(0);
        }
        if (x1 > x2 && op1 == 'L' && op2 == 'R') {
          cout << "Yes" << endl;
          exit(0);
        }
      }
    }
  }
  cout << "No" << endl;
}