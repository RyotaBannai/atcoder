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
  vector<T> bits[2];
  BIT(int n_) { init(n_); }
  void init(int n_)
  {
    n = n_ + 1;
    for (int p = 0; p < 2; p++)
      bits[p].assign(n, 0);
  }

  void add_sub(int p, int i, T x)
  {
    for (int idx = i; idx < n; idx += (idx & -idx)) {
      bits[p][idx] += x;
    }
  }

  // 区間加算 RAQ 対応
  void add(int l, int r, T x) // [l,r) に加算
  {
    add_sub(0, l, -x * (l - 1));
    add_sub(0, r, x * (r - 1));
    add_sub(1, l, x);
    add_sub(1, r, -x);
  }

  auto sum_sub(int p, int i) -> T
  {
    T s(0);
    for (int idx = i; idx > 0; idx -= (idx & -idx)) {
      s += bits[p][idx];
    }
    return s;
  }

  auto sum(int i) -> T { return sum_sub(0, i) + sum_sub(1, i) * i; }

  /*
   [l, r) の区間和は「[1, r)の区間和 – [1, l)の区間和」 で計算できる
   */
  auto query(int l, int r) -> T { return sum(r - 1) - sum(l - 1); }
};

auto main() -> int
{
  auto bit = make_unique<BIT<int>>(10);
  bit->add(3, 6, 2);
  cout << bit->sum(3) << endl;
  cout << bit->query(3, 6) << endl; // (5-3 + 1) * 2
}