/*
Distinct Strings
*/
#include <algorithm>
#include <iostream>
#include <set>
#include <string>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;

vector<char> v;
set<string> s;

void solve(int n)
{
  do {
    string str = "";
    lp(i, n) str += v[i];
    if (s.find(str) == s.end())
      s.insert(str);
  } while (next_permutation(v.begin(), v.end()));
}

auto main() -> int
{
  int n;
  string line;
  cin >> line;
  n = line.size();

  v.reserve(n);
  lp(i, n) v.push_back(line[i]);
  sort(v.begin(), v.end());

  solve(n);

  cout << s.size() << endl;
}