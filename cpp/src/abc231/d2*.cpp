/* @cpg_dirspec d
Neighbors

DFS rec

passed.
*/
#include <iostream>
#include <map>
#include <vector>
using namespace std;
static const int MAXN = 100100;
vector<int> mat[MAXN + 1];
int depth[MAXN + 1];

auto dfs(int s, int d) -> bool
{
  depth[s] = d;
  for (auto x : mat[s]) {
    if (depth[x]) {          // すでに通った
      if (d - 1 == depth[x]) // 親
        continue;
      return false; // ループしている
    }
    if (!dfs(x, d + 1))
      return false;
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