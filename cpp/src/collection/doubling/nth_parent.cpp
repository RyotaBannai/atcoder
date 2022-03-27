/**
ダブリングによるK個先の親を見つける方法：
https://satanic0258.hatenablog.com/entry/2017/02/23/222647

問題:
0 番目は根となる根付き木がある.
0~N 個のそれぞれのノードに一意の番号が付けられている.
ノード C の K 個目の親を出力せよ.

入力:
N M C K (N:ノードの数 M:辺の数 C:ノード K:K 個上の親)
P1 C1 (親 子)
...
Pm Cm (親 子)
の形式で与えられる.
*/

#include <cstdio>
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll N, M, C, K;
  cin >> N >> M >> C >> K;

  // prev: key が子供で値が親
  // next: key が親で値が子供
  vector<int> prev(N); // 親は一人
  prev.at(0) = 0;

  // vector<vector<int>> next(N + 1, vector<int>(N + 1)); // 特に左右区別しない
  for (int i = 0; i < M; ++i) {
    int p, c;
    cin >> p >> c;
    prev.at(c) = p;
    // next.at(p).push_back(c);
  }

  int logK = 1;
  while ((1LL << logK) <= K) {
    logK++;
  }

  // doubling[k][i] : i 番目から 2^k 進んだ親
  vector<vector<int>> doubling(logK, vector<int>(N));
  for (int i = 0; i < N; ++i) {
    doubling[0][i] = prev[i];
  }

  // 前処理
  for (int k = 0; k < logK - 1; ++k) {
    for (int i = 0; i < N; ++i) {
      doubling[k + 1][i] = doubling[k][doubling[k][i]];
    }
  }

  // debug
  // for (int k = 0; k < logK; ++k) {
  //   for (int i = 0; i < N; ++i) {
  //     cout << doubling[k][i] << " ";
  //   }
  //   cout << endl;
  // }

  int now = C;
  for (int k = 0; K > 0; k++) {
    if (K & 1) {
      // cout << k << " " << now << endl;
      now = doubling[k][now];
    }
    K = K >> 1;
  }
  cout << now << endl;
}