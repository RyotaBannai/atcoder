/**
ダブリングによるK個先の親を見つける：

https://satanic0258.hatenablog.com/entry/2017/02/23/222647
 */

#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll N, K;
  cin >> N >> K;
  vector<int> A(N);
  for (int i = 0; i < N; ++i) {
    cin >> A.at(i);
    A[i]--; // 0-indexed
  }

  int logK = 1;
  while ((1LL << logK) <= K) {
    logK++;
  }

  /*
  4 5 // N K
  3 2 4 1
  */
  // cout << "logK: " << logK << endl; // logK = 3

  // doubleing[k][i] : i 番目から 2^k 進んだ町
  vector<vector<int>> doubling(logK, vector<int>(N));
  for (int i = 0; i < N; ++i) {
    doubling[0][i] = A[i];
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

  int now = 0;
  for (int k = 0; K > 0; k++) {
    if (K & 1) {
      // cout << k << " " << now << endl;
      now = doubling[k][now];
    }
    K = K >> 1;
  }
  cout << now + 1 << endl;
}