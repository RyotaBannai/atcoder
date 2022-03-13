/* @cpg_dirspec c
Collision 2

https://atcoder.jp/contests/abc243/tasks/abc243_c

AC
 */
#include <algorithm>
#include <iostream>
#include <map>
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
  map<int, vector<T>> ys;

  for (int i = 0; i < N; ++i) {
    int x, y;
    cin >> x >> y;
    ys[y].emplace_back(x, y, i);
  }

  string S;
  cin >> S;

  /*
  同じ y 座標同士にいるかどうか

  x 座標が交差するかどうか
  一番右にある 左進行のものが、
  一番左にある 右進行のものより x 座標が大きければ交差する
  */

  for (auto item : ys) {
    auto v = item.second;

    int xmax_l = -1;
    int xmin_r = numeric_limits<int>::max();

    for (T x : v) {
      if (S[get<2>(x)] == 'L') {
        xmax_l = max(xmax_l, get<0>(x));
      }

      if (S[get<2>(x)] == 'R') {
        xmin_r = min(xmin_r, get<0>(x));
      }
    }

    if (xmin_r < xmax_l) {
      cout << "Yes" << endl;
      exit(0);
    }
  }
  cout << "No" << endl;
}