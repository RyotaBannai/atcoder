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
  vector<int> dat, lazy;
  RMQ(int n) : n(n), dat(n * 2), lazy(n * 2) {}

  auto left(int i) -> int { return dat[i * 2 + 1]; }
  auto right(int i) -> int { return dat[i * 2 + 2]; }

  void eval(int k)
  {
    if (lazy[k] == INF)
      return;
    if (k < n - 1) {
      lazy[left(k)] = lazy[k];
      lazy[right(k)] = lazy[k];
    }
    dat[k] = lazy[k];
    lazy[k] = INF;
  }

  void update(int i, int v)
  {
    i += n - 1;
    dat[i] = v;
    while (i > 0) {
      i = (i - 1) / 2;
      dat[i] = min(left(i), right(i));
    }
  }

  auto query_sub(int a, int b, int k, int l, int r) -> int
  {
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