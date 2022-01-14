/*
JOI 2017 予選 F L番目のK番目の数
http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0646

長さ N の整数列 a1,a2,…,aN が与えられる。
整数列の連続する部分列のうち長さが K 以上のものそれぞれについて、その中で K
番目に小さい数を書き出していく。
こうして書き出された数のうち L 番目に小さい数を求めよ。
*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

/* BIT */
template <class Abel> struct BIT {
  vector<Abel> dat;
  Abel UNITY_SUM = 0; // to be set

  /* [1, n] */
  BIT(int n) { init(n); }
  void init(int n)
  {
    dat.resize(n + 1);
    for (int i = 0; i < (int)dat.size(); ++i)
      dat[i] = UNITY_SUM;
  }

  /* a is 1-indexed */
  inline void add(int a, Abel x)
  {
    for (int i = a; i < (int)dat.size(); i += i & -i)
      dat[i] = dat[i] + x;
  }

  /* [1, a], a is 1-indexed */
  inline auto sum(int a) -> Abel
  {
    Abel res = UNITY_SUM;
    for (int i = a; i > 0; i -= i & -i)
      res = res + dat[i];
    return res;
  }

  /* [a, b), a and b are 1-indexed */
  inline auto sum(int a, int b) -> Abel { return sum(b - 1) - sum(a - 1); }
};

auto main() -> int
{
  /* 入力 */
  int N, K;
  long long L;
  cin >> N >> K >> L;
  vector<int> a(N);
  for (int i = 0; i < N; ++i)
    cin >> a[i];

  /* 二分探索 */
  int low = 0, high = N;
  while (high - low > 1) {
    /* mid 以下の値が L 個あるかを判定する */
    // mid が増えると言うことは、mid 以下になる数が増えると言うこと
    int mid = low + (high - low) / 2;
    // cout << endl << "mid: " << mid << endl;

    /* しゃくとり法で、mid 以下の値が K 個以上あるような区間を数え上げる */
    BIT<int> bit(N + 1);
    int right = 0;
    long long num = 0;
    for (int left = 0; left < N; ++left) {
      while (right < N && bit.sum(mid) < K) {
        bit.add(a[right], 1);
        ++right;
      }
      // sum(mid) の mid を含めて K 以下あるかどうか見る
      // cout << "bit.sum(mid): " << bit.sum(mid) << endl;
      if (bit.sum(mid) < K)
        break;
      num += N - right + 1; // サブセット数

      // cout << "N: " << N << ", right: " << right << endl;
      // cout << "num: " << num << endl;
      // exit(0);
      bit.add(a[left], -1);
    }

    if (num >= L)
      high = mid;
    else
      low = mid;
  }

  cout << high << endl;
}