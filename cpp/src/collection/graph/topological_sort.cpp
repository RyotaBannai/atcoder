/*
トポロジカルソート 依存関係を解決する処理


無向グラフではサイクルのないグラフは森 (木を集めたもの)
有向グラフではサイクルのないグラフは DAG (Directed Acyclic Graph)

出次数 deg[v] 頂点 v を始点とする辺の個数
シンク　出次数が 0 であるような頂点（つまり他のタスクに依存していない頂点）

シンクに向かって伸びている辺たちを、元のグラフから削除することを繰り返す

*/
#include <iostream>
#include <queue>
#include <vector>

using namespace std;
using Graph = vector<vector<int>>;

auto main() -> int
{
  // N:= 頂点数、M:=辺数
  int N, M;
  cin >> N >> M;

  // グラフ入力受取 (ここでは無向グラフを想定)
  Graph G(N);
  vector<int> deg(N, 0);
  for (int i = 0; i < M; ++i) { // 辺の from - to が与えられる
    int a, b;
    cin >> a >> b;
    G[b].push_back(a); // 辺の逆向きが逆になるように張る
    deg[a]++;          // 出次数 = 依存されてた頂点が依存される側となる
  }

  queue<int> que;
  for (int i = 0; i < N; i++) {
    if (deg[i] == 0) { // シンクたちをキューに入れる
      que.push(i);
    }
  }

  vector<int> order;
  while (!que.empty()) {
    int v = que.front();
    que.pop();
    order.push_back(v);

    for (int nv : G[v]) {
      deg[nv]--;
      if (deg[nv] == 0) {
        que.push(nv);
      }
    }
  }

  // 結果出力 (各頂点の頂点 0 からの距離を見る)
  reverse(order.begin(), order.end());
  for (auto x : order)
    cout << x << endl;
}