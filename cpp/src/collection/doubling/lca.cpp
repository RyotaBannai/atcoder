/**
ダブリングによる最近共通祖先（LCA：Lowest Common Ancestor）を見つける方法：
https://algo-logic.info/lca/#toc_id_2_2

問題:
木の２つのノードの一番近い共通の先祖を見つけよ.

問題:
0 番目は根となる根付き木がある.
0~N 個のそれぞれのノードに一意の番号が付けられている.
ノード U と V の最近共通祖先を出力せよ.

入力:
N M U V (N:ノードの数 M:辺の数 U:ノード1 V:ノード2)
P1 C1 (親 子)
...
Pm Cm (親 子)
の形式で与えられる.
*/

#include <cstdio>
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;
template <class T> using vec2d = vector<vector<T>>;

vector<int> dist; // ノード数分用意
// parent: key が子供で値が親
// child: key が親で値が子供
vector<int> parent; // 親は一人
vec2d<int> child;   // 親の子に対する左右の区別しない

/**
 * @brief 根からの距離と1つ先の頂点を求める
 * @param v 深さにあるノードの番号
 * @param p 現在ノードの親の番号（巡回しないかチェックのために使用）
 * @param d 現在のルートからの距離（深さ）
 */
void dfs(int v, int p, int d)
{
  dist[v] = d;
  for (auto c : child[v]) {
    if (c != p) { // 有効グラフであるが念ため
      dfs(c, v, d + 1);
    }
  }
};


// 2 点間の距離 → dist(u) + dist(v) – 2 × dist(x)
// int get_dist(int u, int v) { return dist[u] + dist[v] - 2 * dist[query(u, v)]; }
// パス上にある頂点があるか → u と a の距離 + a と v の距離 == u と v の距離
// bool is_on_path(int u, int v, int a) { return get_dist(u, a) + get_dist(a, v) == get_dist(u, v); }

auto main() -> int
{
  ll N, M, u, v;
  cin >> N >> M >> u >> v;

  dist.assign(N, 0);
  parent.assign(N, 0);
  child.assign(N, vector<int>{});

  for (int i = 0; i < M; ++i) {
    int p, c;
    cin >> p >> c;
    parent.at(c) = p; // doubling の前処理に使う.
    child.at(p).push_back(c); // ルートからのそれぞれのノードの距離を作る前処理に使う.
  }
  parent.at(0) = 0;

  int logK = 1;
  while ((1LL << logK) <= N) { // N: ノードの最大値分考慮
    logK++;
  }

  // 前処理: doubling
  // doubling[k][i] : i 番目から 2^k 進んだ親
  vec2d<int> doubling(logK, vector<int>(N));
  for (int i = 0; i < N; ++i) {
    doubling[0][i] = parent[i];
  }
  for (int k = 0; k < logK - 1; ++k) {
    for (int i = 0; i < N; ++i) {
      doubling[k + 1][i] = doubling[k][doubling[k][i]];
    }
  }

  // debug
  // for (int k = 0; k < logK; ++k) {
  //   for (int i = 0; i < N; ++i) {
  //     cout << doubling[k][i] << " ";
  //   }
  //   cout << endl;
  // }

  // 前処理: ルートからの距離（深さ）
  dfs(0, 0, 0);

  if (dist[u] < dist[v]) { // u の方が深いようにする
    swap(u, v);
  }
  // LCA までの距離を同じにする
  for (int k = 0; k < logK; k++) {
    // debug
    // cout << "u dist:" << dist[u] << ", node: " << u << endl;
    // cout << "v dist:" << dist[v] << ", node: " << v << endl;
    // cout << "diff dist:" << (dist[u] - dist[v]) << endl;
    /*
    差分を二進展開したものを & 1 してマッチする場合は doubling から移動位置を取り出す
    シフト >> の状態は管理してないが k で >> 1 した状態を取り出しているため、
    作成済みの doubling を利用.

    i.g.
    dist[u] - dist[v] = 3 の場合、
    二進展開すると 011

    two の初期値が 1 でループごとに左に 1 シフトすると
    011 & 1 が true になる場合に two を順に加えると元に戻す事ができる

    ゆえに以下の if の条件では、
    2,4,8 などの場合は一ステップで目的の位置まで移動
    3,6,12 などの場合は二ステップで目的の位置まで移動
    */
    if ((dist[u] - dist[v]) >> k & 1) {
      u = doubling[k][u];
    }
  }

  if (u == v) {
    cout << u << endl;
    exit(0);
  }
  for (int k = logK - 1; k >= 0; k--) {
    if (doubling[k][u] != doubling[k][v]) {
      u = doubling[k][u];
      v = doubling[k][v];
    }
  }
  cout << doubling[0][u] << endl;
}