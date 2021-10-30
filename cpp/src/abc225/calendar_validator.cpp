#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;

const static int ROW = 10000;
const static int COL = 7;

auto main() -> int
{
  vector<int> A(ROW * COL);
  int n, m;
  cin >> n >> m;
  vector<int> B(n + 1 * m + 1);

  lp(i, n) lp(j, m) cin >> B[i * m + j];
  lp(i, ROW) lp(j, COL) A[i * 7 + j] = i * 7 + j + 1;

  auto it = find(A.begin(), A.end(), B[0]);
  int row, col;
  if (it != A.end())
    row = *it / COL, col = (*it % COL) - 1;
  else {
    cout << "No" << endl;
    return 0;
  }

  cout << row << " " << col << endl;
  lp(i, n)
  {
    lp(j, m)
    {
      if (B[i * m + j] != A[(row + i) * 7 + (col + j)]) {
        cout << "No" << endl;
        return 0;
      }
    }
  }

  cout << "Yes" << endl;
  return 0;

  // lp(i, n)
  // {
  //   lp(j, m) cout << B[i * m + j] << " ";
  //   cout << endl;
  // }

  // lp(i, ROW)
  // {
  //   lp(j, COL) cout << A[i * 7 + j] << " ";
  //   cout << endl;
  // }
}