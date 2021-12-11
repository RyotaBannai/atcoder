/*
Counting 2
*/
#include <algorithm>
#include <iostream>
#include <map>
#include <vector>
using namespace std;

auto main() -> int
{
  int N, Q, l, m, cnt = 0;
  vector<int> as;

  cin >> N >> Q;

  for (int i = 0; i < N; i++) {
    cin >> l;
    as.push_back(l);
  }
  sort(as.begin(), as.end());

  for (int i = 0; i < Q; i++) {
    cin >> m;
    int index = std::distance(as.begin(), std::lower_bound(as.begin(), as.end(), m));
    cout << (N - index) << endl;
  }
}