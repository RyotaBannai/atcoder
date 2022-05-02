/* @cpg_dirspec d
Neighbors

Disjoint Set/Union Find

passed.
*/
#include <iostream>
#include <map>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;
static const int MAXN = 100100;
int depth[MAXN + 1];

class DisjointSet {
public:
  vector<int> rank, p;
  DisjointSet() = default;
  DisjointSet(int size)
  {
    rank.resize(size, 0);
    p.resize(size, 0);
    lp(i, size) makeSet(i);
  }
  void makeSet(int x) { p[x] = x, rank[x] = 0; }
  auto same(int x, int y) -> bool { return findSet(x) == findSet(y); }
  void unite(int x, int y) { link(findSet(x), findSet(y)); }
  void link(int x, int y)
  {
    if (rank[x] > rank[y])
      p[y] = x;
    else {
      p[x] = y;
      if (rank[x] == rank[y])
        rank[y]++;
    }
  }
  auto findSet(int x) -> int
  {
    if (x != p[x])
      p[x] = findSet(p[x]);
    return p[x];
  }
};

auto main() -> int
{
  int N, M, a, b;
  cin >> N >> M;
  std::map<int, int> s;
  DisjointSet ds = DisjointSet(N);

  for (int i = 0; i < M; i++) {
    cin >> a >> b;
    s[a]++;
    s[b]++;

    if (ds.same(a, b)) {
      cout << "No" << endl;
      exit(0);
    }
    else {
      ds.unite(a, b);
    }
  }

  for (auto x : s) {
    if (x.second >= 3) {
      cout << "No" << endl;
      exit(0);
    }
  }

  cout << "Yes" << endl;
}