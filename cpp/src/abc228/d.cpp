/*
Linear Probing
*/
#include <iostream>
#include <vector>
#define ll long long
using namespace std;

auto main() -> int
{
  int N = 1 << 20;
  int Q;
  cin >> Q;
  ll A[N + 1];

  for (int i = 0; i < N; i++) {
    A[i] = -1;
  }

  int t;
  ll x;
  while (Q--) {
    cin >> t >> x;
    if (t == 1) {
      int h = x;
      while (A[h % N] != -1) {
        h++;
      }
      A[h % N] = x;
    }
    else {
      cout << (A[x % N]) << endl;
    }
  }
}