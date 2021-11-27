/*
Star or Not
*/
#include <algorithm>
#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;

auto main() -> int
{
  int N, a, b;
  cin >> N;
  vector<int> p(N);

  lp(i, N - 1)
  {
    cin >> a >> b;
    p[a - 1]++, p[b - 1]++;
  }

  cout << (find(p.begin(), p.begin() + N, N - 1) != p.begin() + N ? "Yes" : "No") << endl;
}