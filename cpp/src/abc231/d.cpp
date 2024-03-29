/*
Neighbors

BFS queue

TLE ver.
*/
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <vector>
using namespace std;
static const int MAXN = 100100;
vector<int> mat[MAXN + 1];

auto bfs(int s) -> bool
{
  queue<pair<int, int>> q;
  q.push(make_pair(s, s)); // (子, 親)
  map<int, int> visited;

  while (!q.empty()) {
    auto u = q.front();
    q.pop();

    if (visited[u.first]) // 訪問済み
      return false;
    else
      visited[u.first]++;

    for (auto x : mat[u.first]) {
      if (u.second == x) // 親
        continue;
      q.push(make_pair(x, u.first)); // (子, 親)
    }
  }

  return true;
}

auto main() -> int
{
  int N, M, a, b;
  cin >> N >> M;
  std::map<int, int> s;

  for (int i = 0; i < M; i++) {
    cin >> a >> b;
    s[a]++;
    s[b]++;
    mat[a].push_back(b);
    mat[b].push_back(a);
  }

  for (auto x : s) {
    if (x.second >= 3) {
      cout << "No" << endl;
      exit(0);
    }
  }
  for (int i = 1; i <= N; i++) {
    if (mat[i].size() == 0)
      continue;

    if (!bfs(i)) {
      cout << "No" << endl;
      exit(0);
    }
  }

  cout << "Yes" << endl;
}