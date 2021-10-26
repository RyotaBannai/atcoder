/*
3 10
1 3 5
true (1,1,3,5)

3 9
1 3 5
false
*/
#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;

static const int INF = 1 << 29;

int n, m; // n := くじの数、合計の数
vector<int> v;
vector<int> combs;

void solve()
/*
4 重ループを避ける: 2 重ループで先に 2 回くじを取り出して、そのあまりが存在するかどうか binary
search にかける. n*4 -> n*2 log n の計算量
*/
{
  lp(i, n) lp(j, n)
  {
    int fst = v[i];
    int snd = v[j];
    if (find(combs.begin(), combs.end(), m - fst - snd) != combs.end()) {
      cout << "true" << endl;
      return;
    }
  }
  cout << "false" << endl;
}

auto main() -> int
{
  cin >> n >> m;
  v.reserve(n);
  lp(i, n) cin >> v[i];

  lp(i, n) lp(j, n) combs.push_back(v[i] + v[j]);
  sort(combs.begin(), combs.begin());
  solve();
}