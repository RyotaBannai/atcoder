/*
n=12
a=(4,6,7,8,1,2,110,2,4,12,3,9)
x=25

input:
12 1
4 6 7 8 1 2 110 2 4 12 3 9
25

output:
32

各 left に対して最悪 O(n) かかるので全体で O(n2)

*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int n, Q;
  cin >> n >> Q;
  vector<ll> a(n);
  for (int i = 0; i < n; i++)
    cin >> a[i];

  for (int j = 0; j < Q; j++) {
    ll x;
    cin >> x;

    // 合計値
    ll res = 0;

    for (int left = 0; left < n; left++) {
      ll sum = 0;
      int right = left; // [left, right) の総和が x 以下となる最大の right を求める

      /* sum に a[right] を加えても大丈夫なら right を動かす */
      while (right < n && sum + a[right] <= x) {
        sum += a[right];
        right++;
      }

      /* break した状態で right は条件を満たす最大 */
      res += (right - left);
    }

    cout << res << endl;
  }
}