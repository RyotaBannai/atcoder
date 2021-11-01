#include <algorithm>
#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;
int N;

auto main() -> int
{
  int T;
  cin >> N;
  vector<int> boards(N);

  lp(i, N) cin >> boards[i];
  cin >> T;

  sort(boards.begin(), boards.end());
  int MAX = boards[N - 1];
  int times = MAX / T;
  int cnt = 0;
  for (int i = 1; i <= times + 1; i++) {
    auto l = lower_bound(boards.begin(), boards.end(), (i - 1) * T,
                         [&](int x, int tar) { return x < tar; });
    auto u =
        lower_bound(boards.begin(), boards.end(), i * T, [&](int x, int tar) { return x < tar; });

    if (l != boards.end() && u != l)
      cnt++;
  }
  cout << cnt << endl;
}