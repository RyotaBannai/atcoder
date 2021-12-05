/*
Linear Probing

セグ木
*/
#include <iostream>
#include <vector>
using namespace std;

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
  }

  auto left(int i) -> int { return i * 2 + 1; }
  auto right(int i) -> int { return i * 2 + 2; }

  void update(int i, int v)
  {
    i += n - 1;
    dat[i] = v;
    while (i > 0) {
      i = (i - 1) / 2;
      dat[i] = min(dat[left(i)], dat[right(i)]);
    }
  }

  inline auto operator[](int a) -> pair<int, T> { return query(a, a + 1); }
  auto query(int a, int b) -> pair<int, T> { return query_sub(a, b, 0, 0, n); }
  auto query_sub(int a, int b, int k, int l, int r) -> pair<int, T>
  {
    if (r <= a || b <= l)
      return make_pair(k, INF);
    else if (a <= l && r <= b)
      return make_pair(k, dat[k]);
    else {
      int kl = left(k), kr = right(k);
      pair<int, T> vl = query_sub(a, b, kl, l, (l + r) / 2);
      pair<int, T> vr = query_sub(a, b, kr, (l + r) / 2, r);

      if (vl.second <= vr.second)
        return make_pair(kl, vl.second);
      else
        return make_pair(kr, vr.second);
    }
  }
};

auto main() -> int
{
  int q;
  cin >> q;
  vector<long long> value(SIZE, INF);
  auto seg = make_unique<SegTree<int>>(SIZE);
  while (q--) {
    int t;
    long long x;
    cin >> t >> x;
    if (t == 1) {
      int h = x & MASK;
      pair<int, int> re = seg->query(h, SIZE); // pair<index, value>
      if (re.second != INF)
        seg->update(re.first, INF);
      cout << h << ", " << x << ", fst: " << re.first << endl;
      value[h] = x;
    }
    else {
      cout << value[x & MASK] << '\n';
    }
  }
}
