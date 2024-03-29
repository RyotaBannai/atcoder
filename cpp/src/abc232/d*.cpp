/* @cpg_dirspec d
Weak Takahashi

with queue passed.

*/
#include <iostream>
#include <queue>
#include <string>
#include <vector>

using namespace std;
using P = pair<int, int>;
template <typename T> using vector2d = vector<vector<T>>;
const static int INF = -(1 << 21);

auto main() -> int
// N,M,i,j の処理を間違えないこと.
{
  int N, M;
  cin >> N >> M;
  vector2d<char> maze(N, vector<char>(M));
  vector2d<int> dist(N, vector<int>(M, INF)); // (1,1) 地点からの距離

  for (auto &v : maze) {
    for (char &x : v) {
      cin >> x;
    }
  }

  vector<P> ns{P(0, 1), P(1, 0)}; // x+1 or y+1
  queue<P> que;
  que.push(P(0, 0)); // 0-indexed
  dist[0][0] = 1;

  while (que.size()) {
    P p = que.front();
    que.pop();
    for (P n : ns) {
      int ny = p.first + n.first;
      int nx = p.second + n.second;
      if (ny < N && nx < M && maze[ny][nx] != '#' && dist[ny][nx] == INF) {
        // cout << "y " << ny << ",x " << nx << endl;
        que.push(P(ny, nx));
        dist[ny][nx] = dist[p.first][p.second] + 1;
      }
    }
  }

  int ans = 0;
  for (auto v : dist) {
    for (int x : v) {
      ans = max(ans, x);
    }
  }

  cout << ans << endl;
}