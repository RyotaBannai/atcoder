/* @cpg_dirspec d
Multiply and Rotate
*/
#include <iostream>
#include <queue>
#include <string>
#include <vector>

using namespace std;
using ll = long long;

auto rot(int x) -> int
{
  int y = x, t = 1;
  for (; y > 0; y /= 10) {
    t *= 10;
  }
  return x / 10 + x % 10 * t / 10;
}

auto main() -> int
{
  int n, a;
  cin >> a >> n;
  int d[1000020];
  queue<int> q;
  memset(d, -1, sizeof d);
  d[1] = 0;
  q.push(1);
  while (!q.empty()) {
    int x = q.front();
    q.pop();
    ll nxt = (ll)x * a;   // (ll)(x * a) これだと ll にはならない
    if (nxt < 1000000) {  // 制約以上の数値になった場合は無視
      if (d[nxt] == -1) { // 未訪問
        d[nxt] = d[x] + 1;
        q.push(nxt);
      }
    }
    if (x >= 10 && x % 10 != 0) {
      int y = rot(x);
      if (d[y] == -1) { // 未訪問
        d[y] = d[x] + 1;
        q.push(y);
      }
    }
  }
  cout << d[n] << endl;
}