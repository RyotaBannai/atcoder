/*
Graph Isomorphism

隣接行列を作成して、
頂点の全パターンを切り替えて試す。

x の頂点と y の頂点が完全一致していて、隣接行列が同じ形をしていれば同一型である.
*/
#include <iostream>
#include <string>
#include <vector>
using namespace std;
template <typename T> using vector2d = vector<vector<T>>;

auto main() -> int
{
  int n, m;
  cin >> n >> m;
  vector2d<bool> x(n, vector<bool>(n)), y(n, vector<bool>(n));
  for (int i = 0; i < m; ++i) {
    int a, b;
    cin >> a >> b;
    a -= 1, b -= 1;
    x[a][b] = x[b][a] = true;
  }
  for (int i = 0; i < m; ++i) {
    int c, d;
    cin >> c >> d;
    c -= 1, d -= 1;
    y[c][d] = y[d][c] = true;
  }
  vector<int> p(n);
  for (int i = 0; i < n; i++) {
    p[i] = i;
  }
  int ans = 0;
  do {
    bool ok = true;
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
        if (x[i][j] != y[p[i]][p[j]]) {
          ok = false;
        }
      }
    }
    if (ok) {
      cout << "Yes\n";
      return 0;
    }
  } while (next_permutation(begin(p), end(p)));
  cout << "No\n";
}
