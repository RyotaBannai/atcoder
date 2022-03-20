/**
ダブリングによるK個先の要素の求め方：

前処理：「doubling[k][i] : i 番目の要素から 2^k 先の要素は何か」　を以下の式を利用して計算
    doubling[k+1][i] = doubling[k][doubling[k][i]]

クエリ：前処理した結果を利用して K 個先の要素を求める
    現在地を now として、K を 2 進数として見た時の全ての桁について以下を行う
    K の k 桁目 が 1 ならば now = doubling[k][now] とする


2^k 先の要素が分かっていれば「 “2^k 先の要素" の 2^k 先」を簡単に求めることができるので、
「2^(k+1) 先の要素が何か」を高速に求めることができる


Teleporter

町が N 個ある。町 i から町 Ai に移動することを K 回繰り返す。
町 1 から始めた時、最終的にどの町にたどり着くか？

https://atcoder.jp/contests/abc167/tasks/abc167_d

参考
https://algo-logic.info/doubling/
https://freestylewiki.xyz/fswiki/wiki.cgi?page=%E3%83%80%E3%83%96%E3%83%AA%E3%83%B3%E3%82%B0
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