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
  x1<x2 && S1 = 'R' && S2 = 'L'
  OR
  x1>x2 && S1 = 'L' && S2 = 'R'

  同じ y 座標同士を 組み合わせて上記を総当たりでチェック？
  取り出した item の 3rd で S の操作を取り出す？
  */

  for (auto item : ys) { // 毎回 copy はコストがかかる
    auto v = item.second;
    std::sort(v.begin(), v.end(), [](T a, T b) { return get<0>(a) < get<0>(b); }); // x で昇順ソート
    int len = v.size();

    bool dp[len + 1];
    memset(dp, false, sizeof(dp)); // right を向いていたら true

    for (int i = 0; i < len; ++i) {
      char op = S[get<2>(v[i])];
      if (dp[i] && op == 'L') {
        cout << "Yes" << endl;
        exit(0);
      }

      // 左に一つでも 右向きがあれば以降は全て true. 今回はこの判定を入れなくても AC
      if (dp[i] || op == 'R') {
        dp[i + 1] = true;
      }
    }
  }
  cout << "No" << endl;
}