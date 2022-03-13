/* @cpg_dirspec c
Collision 2

https://atcoder.jp/contests/abc243/tasks/abc243_c

一度に sort したほうが早い
 */
#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <tuple>

using namespace std;
using ll = long long;
using T = tuple<int, int, int>; // x,y,nth

struct cmp {
  auto operator()(T a, T b) const -> bool { return get<0>(a) < get<0>(b); } // x で昇順ソート
};

auto main() -> int
{
  int N;
  cin >> N;
  map<int, set<T, cmp>> ys;

  for (int i = 0; i < N; ++i) {
    int x, y;
    cin >> x >> y;
    ys[y].emplace(x, y, i);
  }

  string S;
  cin >> S;

  for (auto item : ys) {
    auto s = item.second;
    int len = s.size();

    bool dp[len + 1];
    memset(dp, false, sizeof(dp)); // right を向いていたら true

    int i = 0;
    for (auto z : s) {
      char op = S[get<2>(z)];
      if (dp[i] && op == 'L') {
        cout << "Yes" << endl;
        exit(0);
      }

      if (dp[i] || op == 'R') {
        dp[i + 1] = true;
      }
      ++i;
    }
  }
  cout << "No" << endl;
}