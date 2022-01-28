/*
幅優先探索テンプレート

BFS は最短経路を求めることができるアルゴリズムであるというのが大きな特徴
この性質を活用する典型的な応用例として
・パズルを解くための最小手数を求める
・Facebook の友達関係などにおいて、ある人からある人へ何ステップで行けるかを求める
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
  for (int i = 0; i < M; ++i) { // 辺の from - to が与えられる
    int a, b;
    cin >> a >> b;
    G[a].push_back(b);
    G[b].push_back(a);
  }

  vector<int> dist(N, -1);
  queue<int> que;

  dist[0] = 0;
  que.push(0);

  while (!que.empty()) {
    int v = que.front();
    que.pop();

    for (int nv : G[v]) {
      if (dist[nv] != -1) {
        continue; // すでに発見済みの頂点には再訪問しない
      }

      dist[nv] = dist[v] + 1; // キューに入れるときには距離は決定済み
      que.push(nv);
    }
  }

  // 結果出力 (各頂点の頂点 0 からの距離を見る)
  for (int v = 0; v < N; ++v)
    cout << dist[v] << endl;
}
