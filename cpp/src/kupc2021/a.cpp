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
  int times = boards[N - 1] / T;
  int fst = 0;
  lp(i, times + 1)
  {
    // cout << boards[0] << " " << (i + 1) * T << endl;
    if ((i + 1) * T > boards[0]) {
      fst = i;
      break;
    }
  }

  // cout << fst << endl;
  // cout << boards[N - 1] << endl;
  // lp(i, N) cout << boards[i] << " ";
  // cout << endl;
  // lp(i, 7) cout << (i + 1) * T << " ";
  // cout << endl;

  cout << times + 1 - fst << endl;
}