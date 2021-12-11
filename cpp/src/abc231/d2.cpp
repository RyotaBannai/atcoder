/*
 */
#include <algorithm>
#include <iostream>
#include <map>
#include <vector>
using namespace std;

auto main() -> int
{
  int N, M, a, b;
  cin >> N >> M;

  vector<int> v;
  bool ans = true;
  for (int i = 0; i < M; i++) {
    cin >> a >> b;
    auto idx = find(v.begin(), v.end(), a);
    if (idx != v.end()) {}
  }

  // cout << (ans ? "Yes" : "No") << endl;
}