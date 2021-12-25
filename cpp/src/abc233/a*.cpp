/*
10yen Stamp
*/
#include <iostream>
#include <string>
#include <vector>
using namespace std;

auto main() -> int
{
  int X, Y, cnt = 0;
  cin >> X >> Y;
  while (true) {
    if (X < Y) {
      cnt++;
      X += 10;
    }
    else {
      break;
    }
  }
  cout << cnt << endl;
}