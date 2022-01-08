/* @cpg_dirspec d
Prefix K-th Max
*/
#include <iostream>
#include <vector>
using namespace std;

template <typename T> struct BIT {
  int n, cnt = 0;
  vector<T> bit;
  BIT(int leafs, int defa = 0) : n(leafs + 1), bit(n, defa) {}

  void add(int i, T x)
  {
    cnt++;
    for (int idx = i; idx < n; idx += (idx & -idx)) {
      bit[idx] += x;
    }
  }

  auto nth(T w) -> int { return lower_bound(w); }
  auto nth_back(T w) -> int { return lower_bound(cnt - w + 1); }
  auto lower_bound(T w) -> int
  {
    if (w <= 0) {
      return 0;
    }
    else {
      int x = 0, r = 1;
      while (r < n)
        r = r << 1;
      for (int len = r; len > 0; len = len >> 1) {
        if (x + len < n && bit[x + len] < w) {
          w -= bit[x + len];
          x += len;
        }
      }
      return x + 1;
    }
  }
};

auto main() -> int
{
  int N, K;
  cin >> N >> K;
  vector<int> as(N);
  auto bit = make_unique<BIT<int>>(N, 0);

  for (int i = 0; i < N; i++) {
    cin >> as[i];
  }

  for (int i = 0; i < K; i++) {
    bit->add(as[i], 1);
  }
  cout << bit->nth_back(K) << endl;

  for (int i = K; i < N; i++) {
    bit->add(as[i], 1);
    cout << bit->nth_back(K) << endl;
  }
}