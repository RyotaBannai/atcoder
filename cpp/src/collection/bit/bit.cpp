/*
Binary Indexed Tree(BIT)
*/

#include <iostream>
#include <vector>
using namespace std;

/* BIT: 区間和の更新や計算を行う構造体
    初期値は a_1 = a_2 = ... = a_n = 0
    ・add(i,x): a_i += x とする
    ・sum(i): a_1 + a_2 + ... + a_i を計算する
    計算量は全て O(logn)
*/
template <typename T> struct BIT {
  int n;
  vector<T> bit;
  BIT(int n_) : n(n_ + 1), bit(n, 0) {}

  /*
  i と　x が与えられた時、ai に x を加算する
  = i から初めて、i に最後に 1 が立っているビットを加算しながら、最後の i の値に x を加える　
  */
  void add(int i, T x)
  {
    for (int idx = i; idx < n; idx += (idx & -idx)) {
      bit[idx] += x;
    }
  }

  /*
  i から初めて最後に 1 が立っているビットを減算しながら、i の値を合計していく
  */
  T sum(int i)
  {
    T s(0);
    for (int idx = i; idx > 0; idx -= (idx & -idx)) {
      s += bit[idx];
    }
    return s;
  }
};
auto main() -> int {}