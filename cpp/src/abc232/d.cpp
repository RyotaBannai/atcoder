/*
Weak Takahashi
*/
#include <iostream>
#include <queue>
#include <string>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;
using P = pair<int, int>;
const static int INF = 1 << 21;
const static char WALL = '#';

int n, m; // n := 迷路行数, m := 迷路列数
vector<vector<char>> maze;
vector<int> dx = {1, 0}; // 右から反時計回り
vector<int> dy = {0, 1};

auto bfs() -> int
{
  vector<int> d[n + 1]; // d := S から各地点までの最短経路
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < m; j++) {
      d[i].push_back(INF);
    }
  }

  queue<P> que;
  que.push(make_pair(0, 0));
  d[0][0] = 1;

  while (que.size()) {
    P p = que.front();
    que.pop();
    lp(i, 2)
    {
      int nx = p.first + dx[i];
      int ny = p.second + dy[i];
      if (nx < m && ny < n && maze[nx][ny] != WALL && d[nx][ny] == INF) {
        que.push(make_pair(nx, ny));
        d[nx][ny] = d[p.first][p.second] + 1;
      }
    }
  }

  int ans = 0;
  for (auto x : d[n - 1]) {
    ans = max(ans, x);
  }
  for (int i = 0; i < n; i++) {
    ans = max(ans, d[i][m - 1]);
  }
  return ans;
}

auto main() -> int
{
  string line;
  cin >> n >> m;
  maze.reserve(n);
  lp(i, n)
  {
    cin >> line, maze[i].reserve(m);
    lp(j, m) { maze[i][j] = line[j]; }
  };

  cout << bfs() << endl;
  return 0;
}