/*
POJ 3061 Subsequence

*/

#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int Q;
  cin >> Q;
  while (Q--) {
    int n, x;
    cin >> n >> x;
    vector<ll> a(n);
    for (int i = 0; i < n; i++)
      cin >> a[i];

    int res = n + 1; // 区間の長さの最小値 (上限を入れておく)
    int right = 0;
    ll sum = 0;
    for (int left = 0; left < n; left++) {
      /* 総和が x 以上となる最小の right を求める */
      // sum == x ならば sum <= x を満たす最小値がすでに発見されている
      while (right < n && sum < x) {
        sum += a[right];
        right++;
      }

      /* break した状態で right は条件を満たす最小 */

      /* 更新 */
      // これ以上 left を進めてもダメ（= right を進めた結果 n
      // を超過し、これ以上加えられる数値がない）
      if (sum < x)
        break;
      res = min(res, right - left);

      /* left をインクリメントする準備 */
      // 以下の操作は他のしゃくとり法と同様
      if (right == left)
        right++; // right が left に重なったら right も動かす
      else
        sum -= a[left]; // left がインクリメントされて区間の総和が減るため sum から a[left] 引く
    }

    /* res = n+1 の時は解なし */
    if (res < n + 1)
      cout << res << endl;
    else
      cout << 0 << endl;
  }
}