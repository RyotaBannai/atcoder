/*
遅延評価セグメント木
RMQ の更新・加算の操作は、一点の更新を n 回行うことになるので
O(n logn) だけかかってしまうが、これを
O(log n) でできるように改良したのが遅延評価セグメント木

https://algo-logic.info/segment-tree/

*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;
static const int INF = 1 << 20;

template <typename T> struct RMQ {
  int n;
  vector<T> dat, lazy;
  RMQ(int leafs) : dat(leafs * 4, INF), lazy(leafs * 4, INF)
  {
    /*
    必要最低限の最小二分木のメモリを確保 leafs = 7 の時 n = 8 確保するため
    全内部接点は２つの子供を持つ.
    */
    int x = 1;
    while (leafs > x)
      x *= 2;
    n = x;
  }

  auto left(int i) -> int { return dat[i * 2 + 1]; }
  auto right(int i) -> int { return dat[i * 2 + 2]; }

  void eval(int k) // 配列のk番目を更新
  {
    if (lazy[k] == INF) // 更新するものが無ければ終了
      return;
    if (k < n - 1) { // 葉でなければ子に伝搬
      lazy[left(k)] = lazy[k];
      lazy[right(k)] = lazy[k];
    }
    dat[k] = lazy[k]; // 自身を更新
    lazy[k] = INF;
  }

  void update(int a, int b, int v, int k, int l, int r)
  {
    eval(k);
    if (a <= l && r <= b) { // 完全に内側の時
      lazy[k] = v;
      eval(k);
    }
    else if (a < r && l < b) { // 一部区間が被る時
      update(a, b, k * 2 + 1, l, (l + r) / 2);
      update(a, b, k * 2 + 2, (l + r) / 2, r);
      dat[k] = min(left(k), right(k));
    }
  }

  auto query_sub(int a, int b, int k, int l, int r) -> int
  {
    eval(k); // 追加
    if (r <= a || b <= l)
      return INF;
    else if (a <= l && r <= b)
      return dat[k];
    else {
      int vl = query_sub(a, b, k * 2 + 1, l, (l + r) / 2);
      int vr = query_sub(a, b, k * 2 + 2, (l + r) / 2, r);
      return min(vl, vr);
    }
  }
  auto query(int a, int b) -> int { return query_sub(a, b, 0, 0, n); }
};

auto main() -> int { return 0; }