/*
On and Off
*/
#include <iostream>
#include <vector>
using namespace std;

int S, T, X;
auto solve() -> bool
{
  if (S < T) {
    return S <= X && X <= T;
  }
  else {
    return S <= X || X < T;
  }
}

auto main() -> int
{
  cin >> S >> T >> X;

  cout << (solve() ? "Yes" : "No") << endl;
}