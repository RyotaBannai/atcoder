/*
Graph Isomorphism

この実装だと以下のグラフが同型になってしまう.
1.
・・・・
　　・
2.
　　・
・・・
　　・

*/
#include <algorithm>
#include <iostream>
#include <set>
#include <vector>
using namespace std;
template <typename T> using vector2d = vector<vector<T>>;

auto main() -> int
{
  int N, M;
  cin >> N >> M;
  vector2d<int> x(N, vector<int>(N, 0)), y(N, vector<int>(N, 0));
  set<int> xs, ys;

  for (int i = 0; i < M; i++) {
    int a, b;
    cin >> a >> b;
    a -= 1, b -= 1;
    x[a][b] = x[b][a] = 1;
  }

  for (int i = 0; i < M; i++) {
    int a, b;
    cin >> a >> b;
    a -= 1, b -= 1;
    y[a][b] = y[b][a] = 1;
  }

  for (int i = 0; i < N; i++) {
    int cnt = 0;
    for (auto a : x[i])
      cnt += a;
    xs.insert(cnt);

    cnt = 0;
    for (auto a : y[i])
      cnt += a;
    ys.insert(cnt);
  }

  cout << (xs == ys ? "Yes" : "No") << endl;
}