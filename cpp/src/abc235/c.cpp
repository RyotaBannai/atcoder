/*
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N, Q;
  cin >> N >> Q;
  vector<int> A(N);
  for (int i = 0; i < N; i++)
    cin >> A[i];

  while (Q--) {
    bool found = false;
    int x, k;
    cin >> x >> k;
    for (int i = 0; i < A.size(); i++) {
      if (x == A[i]) {
        k--;
        if (!k) {
          cout << (i + 1) << endl;
          found = true;
        }
      }
    }
    if (!found)
      cout << (-1) << endl;
  }
}