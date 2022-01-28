/* 二部グラフ判定
https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_7_A&lang=jp
*/

#include <iostream>
#include <queue>
#include <vector>
using namespace std;
using Graph = vector<vector<int>>;

auto main() -> int
{
  int X, Y, M, V;
  cin >> X >> Y >> M;
  V = X + Y;

  // グラフ入力受取
  Graph G(V);
  for (int i = 0; i < M; ++i) {
    int a, b;
    cin >> a >> b;
    b += X;
    G[a].push_back(b);
    G[b].push_back(a);
  }

  // int y = 0;
  // for (auto e : G) {
  //   cout << y++ << ": ";
  //   for (auto u : e) {
  //     cout << u << " ";
  //   }
  //   cout << endl;
  // }
  // exit(0);

  int ans = 0;
  vector<int> dist(V, -1);
  queue<int> que;
  for (int v = 0; v < V; ++v) {
    if (dist[v] != -1) { // v が探索済みならスルー
      continue;
    }
    dist[v] = 0, que.push(v);
    while (!que.empty()) {
      int v = que.front();
      que.pop();
      for (auto nv : G[v]) {
        if (dist[nv] == -1) {
          dist[nv] = dist[v] + 1;
          que.push(nv);
        }
        else {
          // 整合性を確認する
          if (dist[v] % 2 != dist[nv] % 2) {
            ans++;
          }
        }
      }
    }
  }

  cout << ans << endl;
}
