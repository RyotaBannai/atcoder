/*
A Reverse
*/
#include <iostream>
#include <string>
#include <vector>
using namespace std;

auto main() -> int
{
  int L, R;
  string S;
  cin >> L >> R >> S;
  std::reverse(begin(S) + (L - 1), begin(S) + R);
  cout << S << endl;
}