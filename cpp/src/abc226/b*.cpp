/*
Counting Arrays
*/
#include <iostream>
#include <set>
#include <string>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;

set<string> s;

auto main() -> int
{
  int N;
  string rest;
  cin >> N;
  getline(std::cin, rest);

  lp(i, N)
  {
    string line;
    getline(std::cin, line);
    if (line[0] == '0' || line.length() <= 2)
      continue;
    s.insert(line.substr(2));
  }

  cout << s.size() << endl;
}