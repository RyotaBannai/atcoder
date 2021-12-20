/*
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
  string line;
  vector2d<char> maze(N, vector<char>(M));
  vector2d<int> dist(N, vector<int>(M, INF));

  for (int i = 0; i < N; i++) {
    cin >> line;
    for (int j = 0; j < M; j++)
      maze[i][j] = line[j];
  };

  queue<P> que;
  que.push(P(0, 0)); // 0-indexed
  dist[0][0] = 1;

  while (que.size()) {
    P p = que.front();
    que.pop();

    vector<P> ns{P(0, 1), P(1, 0)};
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
  for (int i = 0; i < N; i++) {
    for (int j = 0; j < M; j++) {
      ans = max(ans, dist[i][j]);
    }
  }

  cout << ans << endl;
}