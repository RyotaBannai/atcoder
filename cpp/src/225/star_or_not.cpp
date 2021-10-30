#include <algorithm>
#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;
static const int INF = 1 << 21;
static const int MAX = 100000;
int N;

vector<vector<int>> mat;
vector<int> p(MAX);

auto solve() -> bool
{
  lp(i, N) p[i] = 0;
  lp(i, N) lps(j, i + 1, N)
  {
    if (mat[i][j] == 1)
      p[i]++, p[j]++;
  }

  return find(p.begin(), p.begin() + N, N - 1) != p.begin() + N;
}

auto main() -> int
{
  int a, b;
  cin >> N;

  mat.reserve(N);
  lp(i, N)
  {
    mat[i].reserve(N - 1);
    lp(j, N - 1) mat[i][j] = INF;
  }
  lp(i, N - 1) cin >> a >> b, mat[a - 1][b - 1] = 1;

  cout << (solve() ? "Yes" : "No") << endl;
}