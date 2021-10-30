/*
迷路探索

10 10
#S######.#
......#..#
.#.##.##.#
.#........
##.##.####
....#....#
.#######.#
....#.....
.####.###.
....#...G#

22

*/
#include <iostream>
#include <queue>
#include <string>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;
using P = pair<int, int>;
P S, G;
const static int INF = 1 << 21;
const static char START = 'S';
const static char GOAL = 'G';
const static char WALL = '#';
const static char PATH = '.';

int n, m; // n := 迷路行数, m := 迷路列数
vector<vector<char>> maze;
vector<vector<int>> d;          // d := S から各地点までの最短経路
vector<int> dx = {1, 0, -1, 0}; // 右から反時計回り
vector<int> dy = {0, 1, 0, -1};

auto bfs() -> int
{
  queue<P> que;
  d.reserve(n);
  lp(i, n)
  {
    d[i].reserve(m);
    lp(j, m) d[i][j] = INF;
  }

  que.push(S);
  d[S.first][S.second] = 0;

  while (que.size()) {
    P p = que.front();
    que.pop();
    if (p.first == G.first && p.second == G.second)
      break;
    lp(i, 4)
    /*
    以下のようにベクトルを取り出さないこと. これは座標 p に対する左右上下ではななく、
    座標 p の x 座標について x = {-1,0,1} 分移動した点に対し y = {-1,0,1} を試している
    for (auto x : dx) for (auto y : dy) { ... }
    */
    {
      int nx = p.first + dx[i];
      int ny = p.second + dy[i];
      if (nx >= 0 && nx < m && ny >= 0 && ny < n && maze[nx][ny] != WALL && d[nx][ny] == INF) {
        que.push(make_pair(nx, ny));
        d[nx][ny] = d[p.first][p.second] + 1;
      }
    }
  }
  return d[G.first][G.second];
}

auto main() -> int
{
  int cnt = 0;
  string line;
  cin >> n >> m;
  maze.reserve(n);
  lp(i, n)
  {
    cin >> line, maze[i].reserve(m);
    lp(j, m)
    {
      maze[i][j] = line[j];
      if (line[j] == START)
        S = make_pair(i, j);
      else if (line[j] == GOAL)
        G = make_pair(i, j);
    }
  };

  // lp(i, n)
  // {
  //   lp(j, m) cout << maze[i][j];
  //   cout << endl;
  // }
  // cout << S.first << S.second << endl;
  // cout << G.first << G.second << endl;

  cout << bfs() << endl;

  // lp(i, n)
  // {
  //   lp(j, m) printf("%10d,", d[i][j]);
  //   cout << endl;
  // }
}