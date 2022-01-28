/* 二部グラフ判定

無効グラフが二部グラフ（bipartile graph）かどうか判定する

二部グラフ:= 以下の条件を満たすようぬ全頂点を白または黒で塗り分けられるグラフのこと
・白頂点同士が辺で結ばれることはなく、黒頂点同士が辺で結ばれることもない
言い換えると、全ての辺は異なるカテゴリ間（辺の一方が白で、もう一方が黒）を結んでいる状態

実装方法：
.適当な頂点 s を選択して、白で塗る
.s から１ステップで到達可能な頂点を黒く塗る
.さらに、その１ステップで到達した頂点から１ステップで到達可能な頂点を白く塗る
...
を繰り返す.

実際には、到達済みかどうかを判定する部分で、隣り合う辺と dist を比較する
これは、v と nv とが隣接しているとき、dist[v] % 2 と dist[nv] % 2
とが異なっているかどうかを確認すればよいことになる.

BFS では隣り合う頂点の差は１以下になるため、その隣り合う辺同士について
dist[v] != dist[nv] が成立するかどうかのみを確認すればよい　
*/

#include <iostream>
#include <queue>
#include <vector>
using namespace std;
using Graph = vector<vector<int>>;

auto main() -> int
{
  // 頂点数と辺数
  int N, M;
  cin >> N >> M;

  // グラフ入力受取
  Graph G(N);
  for (int i = 0; i < M; ++i) {
    int a, b;
    cin >> a >> b;
    G[a].push_back(b);
    G[b].push_back(a);
  }

  bool is_bipartite = true;
  vector<int> dist(N, -1);
  queue<int> que;
  for (int v = 0; v < N; ++v) {
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
          if (dist[v] == dist[nv])
            is_bipartite = false;
        }
      }
    }
  }

  if (is_bipartite)
    cout << "Yes" << endl;
  else
    cout << "No" << endl;
}
