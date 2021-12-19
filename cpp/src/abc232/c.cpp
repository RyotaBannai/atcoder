/*

*/
#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

auto main() -> int
{
  int N, M;
  cin >> N >> M;
  vector<int> takahashi[N + 1], aoki[N + 1];
  vector<int> ts, as;

  for (int i = 0; i < M; i++) {
    int a, b;
    cin >> a >> b;
    takahashi[a].push_back(b);
    takahashi[b].push_back(a);
  }

  for (int i = 0; i < M; i++) {
    int a, b;
    cin >> a >> b;
    aoki[a].push_back(b);
    aoki[b].push_back(a);
  }

  for (int i = 1; i <= N; i++) {
    ts.push_back(takahashi[i].size());
    as.push_back(aoki[i].size());
  }

  sort(ts.begin(), ts.end());
  sort(as.begin(), as.end());

  bool ans = true;

  for (int i = 0; i < N; i++) {
    ans &= (ts[i] == as[i]);
  }

  cout << (ans ? "Yes" : "No") << endl;
}