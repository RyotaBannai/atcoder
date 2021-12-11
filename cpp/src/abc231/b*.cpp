/*
Election
*/
#include <iostream>
#include <map>
#include <vector>
using namespace std;

auto main() -> int
{
  string q;
  int N;
  cin >> N;
  std::map<string, int> s;
  for (int i = 0; i < N; i++) {
    cin >> q;
    s[q]++;
  }

  int ans = 0;
  string name;
  for (auto x : s) {
    if (x.second > ans) {
      ans = x.second;
      name = x.first;
    }
  }

  cout << name << endl;
}