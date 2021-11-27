/*
Longest X
*/
#include <iostream>
#include <vector>
using namespace std;

string S;
int K;

auto rec(string s, int idx, int cnt, int k) -> int
{
  if (s.size() <= idx)
    return cnt;

  int whenX = 0, whenNotX = 0;
  if (s[idx] == 'X') {
    whenX = rec(s, idx + 1, cnt + 1, k);
  }
  else if (k > 0) {
    whenNotX = max(rec(s, idx + 1, cnt + 1, k - 1), rec(s, idx + 1, 0, k));
  }
  return max(cnt, max(whenX, whenNotX));
}

auto main() -> int
{
  cin >> S >> K;
  cout << rec(S, 0, 0, K) << endl;
}