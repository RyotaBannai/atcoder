/*
サイクル検出　無効グラフにおいてサイクルになる頂点が存在するかどうか検出する

出次数 deg[v] 頂点 v を始点とする辺の個数
シンク　出次数が 0 であるような頂点（つまり他のタスクに依存していない頂点）

サイクルに含まれる頂点は一度もシンクになることはあり得ないため、キューに挿入されることはない
（サイクルになるということは、出次数が 0 になることはないということ）

*/
#include <iostream>
#include <queue>
#include <vector>

using namespace std;
using Graph = vector<vector<int>>;

auto main() -> int
{
  // N:=辺数
  int N;
  cin >> N;

  // グラフ入力受取 (ここでは無向グラフを想定)
  Graph G(N);
  vector<int> deg(N, 0);
  for (int i = 0; i < N; ++i) { // 辺の from - to が与えられる
    int a, b;
    cin >> a >> b, a--, b--; // 0-indexed にする
    G[a].push_back(b);
    G[b].push_back(a);
    deg[a]++, deg[b]++; // 無向グラフ
  }

  queue<int> que;
  for (int i = 0; i < N; i++) {
    if (deg[i] == 1) { // シンクたちをキューに入れる
      que.push(i);
    }
  }

  // 無効グラフの場合は、deg[v]==1 が葉の条件
  vector<bool> isPushed(N, false);
  while (!que.empty()) {
    int v = que.front();
    que.pop();
    isPushed[v] = true;

    for (int nv : G[v]) {
      deg[nv]--;
      if (deg[nv] == 1) {
        que.push(nv);
      }
    }
  }

  // 結果出力 (各頂点の頂点 0 からの距離を見る)
  int Q;
  cin >> Q;
  while (Q--) {
    int a, b;
    cin >> a >> b, a--, b--;
    // どちらもの頂点も一度も push されていない
    cout << (!isPushed[a] && !isPushed[b] ? 2 : 1) << endl;
  }
}