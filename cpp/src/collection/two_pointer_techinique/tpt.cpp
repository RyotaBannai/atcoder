/*
AOJ  DSL_3_C

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

    ll res = 0;    // 合計値
    ll sum = 0;    // 毎回 sum を使い回すようにする
    int right = 0; // right も使い回す
    for (int left = 0; left < n; left++) {
      /* sum に a[right] を加えても大丈夫なら right を動かす */
      while (right < n && sum + a[right] <= x) {
        sum += a[right];
        right++;
      }

      /* break した状態で right は条件を満たす最大 */
      res += (right - left);
      /* left をインクリメントする準備 */
      if (right == left)
        right++; // right が left に重なったら right も動かす
      else
        sum -= a[left]; // left がインクリメントされて区間の総和が減るため sum から a[left] 引く
    }

    cout << res << endl;
  }
}