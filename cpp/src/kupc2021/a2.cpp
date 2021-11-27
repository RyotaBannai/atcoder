/*
Standing Sign
*/
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

  bool standing = false;
  int visits = 0;
  for (int i = 1; i <= boards[N - 1]; i++) {
    auto res = find_if(boards.begin(), boards.end(), [&](int x) { return x == i; });
    standing |= res != boards.end();
    if (i % T == 0 && standing) {
      // cout << "in: " << i % T << endl;
      standing = false, visits++;
    }
  }
  cout << visits + (standing ? 1 : 0) << endl;
}