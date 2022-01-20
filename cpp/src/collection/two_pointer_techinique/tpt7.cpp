/*
ABC 017 D サプリメント
https://beta.atcoder.jp/contests/abc017/tasks/abc017_4

1 以上 M 以下の整数からなる長さ N の数列 f1,f2,…,fn が与えられる。
数列を前から順番に区切って行く。各区間は「同じ値が二度以上登場しない」という条件を満たすようにしたい。
そのような方法が何通りあるかを 1000000007 で割った余りで求めよ。
*/
#include <iostream>
#include <iterator>
#include <vector>
#define val_to_string(Variable) (#Variable)

using namespace std;
using ll = long long;

template <typename T> void show(vector<T> var)
{
  for (auto x : var)
    cout << x << " ";
  cout << endl;
}

template <typename T> auto slice(vector<T> v, int start, int end = -1) -> vector<T>
{
  if (end == -1) {
    end = v.size();
  }
  auto it = v.begin();
  advance(it, end);
  if (it <= v.end()) { // end が範囲内
    return vector<T>(v.begin() + start, v.begin() + end);
  }
  else { // end が範囲外の場合は、end
    return vector<T>(v.begin() + start, v.end());
  }
}

const int MOD = 1000000007;
vector<int> dp(110000);
vector<int> sum(110000);

auto main() -> int
{
  int N, M;
  cin >> N >> M;
  vector<int> f(N); // f=(1,2,1,2,2)
  for (int i = 0; i < N; ++i)
    cin >> f[i], --f[i];

  /* しゃくとり法
  . L は重複を許さない時の left の位置で、その index がその時の right の位置
  L[right] = left
  */
  vector<int> fnum(M, 0);  // 区間に種類 i が何個あるか
  vector<int> L(N + 1, 0); // 各 i に対するしゃくとり法の区間
  int left = 0;
  for (int right = 0; right < N; ++right) {
    fnum[f[right]]++;

    // cout << val_to_string(fnum) << endl, show(fnum);

    while (left < right && fnum[f[right]] > 1) {
      --fnum[f[left]];
      ++left;
    }
    L[right + 1] = left; // 1-indexed. この後の計算で使う漸化式で初めは 1 が必要

    // cout << val_to_string(L) << endl, show(L);
  }

  /* 累積和で高速化した DP
  . dp[i] := 最初の i 個のサプリまで吸収する方法の数
  . dp は常に i 個目を吸収した時点の組合せ数を表す
  . r が 1 増えると、組み合わせは r-1 の時の 2 倍になる. その数に対して、重複を許さない left
  までの合計を引くと実際の組合せ数が分かる.
  (. 基本的な考え方は、dp そのものをベースにして、例えば、
  left,right = 5,3 の時 1,3 区間のパターン数に対して、4,5
  区間が加わった時にできるパターン数を求めると言うこと)
  */
  dp[0] = 0; // 0 or 1 doesn't really matter.
  sum[0] = 0;
  sum[1] = 1;
  for (int r = 1; r <= N; ++r) {
    dp[r] = (sum[r] - sum[L[r]] + MOD) % MOD; // DP

    // cout << "dp[" << r << "] " << dp[r] << " = sum[" << r << "] (" << sum[r] << ") - sum[" <<
    // L[r]
    //      << "] (" << sum[L[r]] << ")" << endl;

    sum[r + 1] = (sum[r] + dp[r]) % MOD; // 累積和

    // cout << "sum[" << (r + 1) << "] " << sum[r + 1] << " = sum[" << r << "] (" << sum[r]
    //      << ") + dp[" << r << "] (" << dp[r] << ")" << endl
    //      << endl;
  }

  cout << dp[N] << endl;
}