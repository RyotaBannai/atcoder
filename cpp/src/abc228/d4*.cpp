/*
Linear Probing

セグ木
*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

constexpr int SIZE = 1 << 20;
constexpr int MASK = SIZE - 1;
constexpr int INF = numeric_limits<int>::max();

template <typename T> struct SegTree {
  int n;
  vector<T> dat;
  SegTree(int leafs) : dat(leafs * 4, INF)
  {
    int x = 1;
    while (leafs > x)
      x *= 2;
    n = x;

    for (int i = 0; i < SIZE; i++)
      set(i, i);
    build();
  }

  auto left(int i) -> int { return i * 2 + 1; }
  auto right(int i) -> int { return i * 2 + 2; }

  void set(int i, T x) { dat[i + n - 1] = x; }
  void build()
  {
    for (int k = n - 2; k >= 0; k--)
      dat[k] = min(dat[left(k)], dat[right(k)]);
  }

  void update(int i, int v)
  {
    i += n - 1;
    dat[i] = v;
    while (i > 0) {
      i = (i - 1) / 2;
      dat[i] = min(dat[left(i)], dat[right(i)]);
    }
  }

  inline auto operator[](int a) -> int { return query(a, a + 1); }
  auto query(int a, int b) -> int { return query_sub(a, b, 0, 0, n); }
  auto query_sub(int a, int b, int k, int l, int r) -> int
  {
    if (r <= a || b <= l)
      return INF;
    else if (a <= l && r <= b)
      return dat[k];
    else {
      int vl = query_sub(a, b, left(k), l, (l + r) / 2);
      int vr = query_sub(a, b, right(k), (l + r) / 2, r);
      return min(vl, vr);
    }
  }
  /* debug */
  void print(int end = 0)
  {
    end = end ?: 2 * n - 1;
    for (int i = 0; i < end; ++i) {
      cout << (*this)[i].second;
      if (i != n)
        cout << ",";
    }
    cout << endl;
  }
};

auto main() -> int
{
  int q;
  cin >> q;
  vector<ll> value(SIZE, -1);
  auto seg = make_unique<SegTree<ll>>(SIZE);
  while (q--) {
    int t;
    ll x;
    cin >> t >> x;
    if (t == 1) {
      int h = x & MASK;
      int re = seg->query(h, SIZE);
      if (re == INF)
        re = seg->query(0, h);

      if (re != INF) {
        seg->update(re, INF);
        value[re] = x;
      }
    }
    else
      cout << value[x & MASK] << '\n';
  }
}
