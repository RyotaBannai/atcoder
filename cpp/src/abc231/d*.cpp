/*
Neighbors

DFS stack

passed.
*/
#include <iostream>
#include <map>
#include <stack>
#include <vector>
using namespace std;
static const int MAXN = 100100;
vector<int> mat[MAXN + 1];
int depth[MAXN + 1];

auto dfs(int s, int d) -> bool
{
  stack<pair<int, int>> stk;
  stk.push(make_pair(s, d));

  while (!stk.empty()) {
    auto u = stk.top();
    stk.pop();
    depth[u.first] = u.second;

    for (auto x : mat[u.first]) {
      if (depth[x]) {
        if (u.second - 1 == depth[x]) // 親
          continue;
        return false;
      }
      stk.push(make_pair(x, u.second + 1));
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
    if (depth[i])
      continue;
    if (!dfs(i, 1)) {
      cout << "No" << endl;
      exit(0);
    }
  }

  cout << "Yes" << endl;
}