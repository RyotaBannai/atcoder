/*
Binary Indexed Tree(BIT)
*/

#include <iostream>
#include <vector>
using namespace std;

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
};
auto main() -> int {}