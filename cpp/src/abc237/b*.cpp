/* @cpg_dirspec b
Matrix Transposition
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int H, W;
  cin >> H >> W;
  int A[H][W];

  for (int i = 0; i < H; i++) {
    for (int j = 0; j < W; j++) {
      cin >> A[i][j];
    }
  }

  // for (int i = 0; i < H; i++) {
  //   for (int j = 0; j < W; j++) {
  //     cout << A[i][j] << " ";
  //   }
  //   cout << endl;
  // }

  for (int j = 0; j < W; j++) {
    for (int i = 0; i < H; i++) {
      cout << A[i][j] << " ";
    }
    cout << endl;
  }
}