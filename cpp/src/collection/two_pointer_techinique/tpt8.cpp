/*
JOI 2013 本選 C バームクーヘン
https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0600

環状のバームクーヘンを 3 つに分けたい。
バームクーヘンは下図 (問題文から引用) のように N 個のピースに分かれていて、
それぞれのサイズは A1,A2,…,AN となっている。
バームクーヘンに 3 箇所切り込みを入れて、3 つの連続するピース列に分割する。
その 3 つのピースサイズの最小値が最大になるようにせよ。
*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N;
  cin >> N;
  vector<ll> a(N * 2); // 環状の数列を扱う時には二周確保はよくやる
  ll total = 0;        // バームクーヘンの全体のサイズ
  for (int i = 0; i < N; i++) {
    cin >> a[i], a[i + N] = a[i], total += a[i];
  }

  /* 二分探索 */
  ll low = 0, high = 1LL << 60;
  while (high - low > 1) {
    ll mid = low + (high - low) / 2;

    /* しゃくとり法により、各切れ目から mid 以上になる最小区間がどこまでかを求める */
    // 必ずしも 0 から始まった時に最適化できるとは限らないため 2 周を許容する
    // -> 5~2 の範囲で切る場合などがある

    vector<int> Next(N, -1);
    vector<ll> Size(N, -1);
    int right = 0;
    ll sum = 0;
    for (int left = 0; left < N; left++) {
      while (right < N * 2 && sum < mid) {
        sum += a[right];
        right++; // ちょうど break した時に right が次のピースの始まりになる
      }
      if (sum >= mid) {
        Next[left] = right; // mid 以上になるときの left ~ right の範囲を記憶
        Size[left] = sum;
      }
      if (left == right)
        // right のピースが大きすぎたりするとそれだけで mid 以上になる場合で、left == right になる
        right++;
      else
        sum -= a[left]; // left を一つずらす
    }

    /* check */
    bool ok = false;
    for (int i = 0; i < N; i++) {
      /* 1 ピース目 */
      int ni = Next[i];
      // i==開始地点, この開始地点（left）に対する終了地点(right) があるかどうかチェック
      if (ni == -1)
        continue;
      if (Size[i] >= total)
        continue;

      /* 2 ピース目 */
      ni %= N; // 二周目に突入している可能性があるため一周目の地点として考慮
      int nni = Next[ni]; // ちょうど right で切れた地点が次のスタート地点
      if (nni == -1)
        continue;

      // 残りが mid 以上なら OK
      if (total - Size[i] - Size[ni] >= mid)
        ok = true;
    }
    if (!ok)
      high = mid;
    else
      // 分割成功なら、一つあたりの大きさをあげても成功する可能性があるため、low を mid にして、次の
      // mid を大きくする
      low = mid;
  }

  cout << low << endl;
}