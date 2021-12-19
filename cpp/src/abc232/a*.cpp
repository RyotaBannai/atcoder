/*

*/
#include <iostream>
#include <string>
#include <vector>
using namespace std;

auto main() -> int
{
  string s;
  cin >> s;
  cout << ((s[0] - '0') * (s[2] - '0')) << endl;
}