/*
Triple Metre
*/
#include <iostream>
using namespace std;

auto main() -> int
{
  string S, s = "oxxoxxoxxoxxoxx"; // 10 文字 + 3*2 くらい 用意
  cin >> S;
  cout << ((s.find(S, 0) != string::npos) ? "Yes" : "No") << endl;
}