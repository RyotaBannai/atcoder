#include <iostream>
#include <vector>
using namespace std;

auto main() -> int
{
  int Q;
  cin >> Q;
  int A[Q + 1];

  for (int i = 0; i <= Q; i++) {
    A[i] = -1;
  }

  int t, x;
  for (int i = 1; i <= Q; i++) {
    cin >> t >> x;
    if (t == 1) {
      int h = x;
      int idx = -1;
      while ((idx = h % Q) != 0 && A[idx] != -1) {
        h++;
      }
      A[h % Q] = x;
    }
    else {
      cout << (A[x % Q]) << endl;
    }
  }
}