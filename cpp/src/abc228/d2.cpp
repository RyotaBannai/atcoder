#include <iostream>
#include <vector>
using namespace std;

constexpr int SIZE = 1 << 20;
constexpr int MASK = SIZE - 1;

auto main() -> int
{
  int q;
  cin >> q;
  vector<int> parent(SIZE);
  vector<long long> value(SIZE, -1);
  fill(begin(parent), end(parent), 0);
  auto find = [&](auto &&self, int x) -> int {
    if (parent[x] == x) {
      return x;
    }
    else {
      return parent[x] = self(self, parent[x]);
    }
  };
  while (q--) {
    int t;
    long long x;
    cin >> t >> x;
    if (t == 1) {
      int i = find(find, x & MASK);
      value[i] = x;
      parent[i] = find(find, (i + 1) & MASK);
    }
    else {
      cout << value[x & MASK] << '\n';
    }
  }
}