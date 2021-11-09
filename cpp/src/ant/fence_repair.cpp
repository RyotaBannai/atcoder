/*
ある長さ L の板から N 個の板を切り出したい
板を切断する際には、その板の長さ分だけのコストがかかる。
例えば、長さ 21 の板から 5,8,8 の 3 つの板を切り出したい時、
元に 21 を 8, 13 に分断するとコスト 21 かかる.
さらに、13 を 5, 8 に分断する時にもコスト 13 かかる.
この時 21 の板から 5,8,8 に分断する際に必要な最小のコストは 33 (13+8) となる

どれだけのコストで全ての板を分断することができるか.
L は与えられない.
N 個の板の長さが与えられる.

in:
3
8 5 8

out:
34
*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N;
  cin >> N;
  vector<int> L(N);

  for (int i = 0; i < N; i++)
    cin >> L[i];

  ll ans = 0;
  // N 個の板を 1 枚につなげるまで続ける.
  while (N > 1) {
    int m1 = 0, m2 = 1;
    if (L[m1] > L[m2])
      swap(m1, m2);

    // sort しても上手くいくが、最も小さい２要素を選択して、合併したものを L い入れると次の回で sort
    // しなおさないといけないためコストがかかる
    // ここでは全体を sort することが目的ではなく, 最も小さい２要素を選択することが目的で,
    // この実装の方が sort するよりコストがかからない
    for (int i = 2; i < N; i++) {
      if (L[i] < L[m1])
        m2 = m1, m1 = i;
      else if (L[i] < L[m2])
        m2 = i;
    }

    // N 個の板のうち一番短い板と、その次に短い板を合併
    int t = L[m1] + L[m2];
    ans += t;

    // デフォルトで m2 に L の最後の要素を次の回のために格納しなおすが
    // m1 が N-1 だと新しい要素、t を N-1 に入れて、さらに t を m2 に入れてしまうため
    // m1 が新しい要素で、一番最後の要素が m2 に来るように調整する
    if (m1 == N - 1)
      swap(m1, m2);
    L[m1] = t;
    L[m2] = L[N - 1];
    N--;
  }
  cout << ans << endl;
  ;
}