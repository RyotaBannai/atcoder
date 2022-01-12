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

const int MOD = 1000000007;
vector<int> dp(110000);
vector<int> sum(110000);

template <typename T> void show(vector<T> var)
{
  for (auto x : var)
    cout << x << " ";
  cout << endl;
}

/**
 * @brief return sliced vector from original vector
 *
 * @tparam T
 * @param v          original vector
 * @param start      0-indexed
 * @param end        size + 1
 * @return vector<T> new sliced vector
 */
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

auto main() -> int
{
  int N, M;
  cin >> N >> M;
  // f=(1,2,1,2,2)
  vector<int> f(N);
  for (int i = 0; i < N; ++i)
    cin >> f[i], --f[i];

  /* しゃくとり法 */
  vector<int> fnum(M, 0);  // 区間に種類 i が何個あるか
  vector<int> L(N + 1, 0); // 各 i に対するしゃくとり法の区間
  int left = 0;
  for (int right = 0; right < N; ++right) {
    fnum[f[right]]++;
    // debug
    // cout << val_to_string(fnum) << endl;
    // show(fnum);

    while (left < right && fnum[f[right]] > 1) {
      --fnum[f[left]];
      ++left;
    }
    L[right + 1] = left;
    // debug
    // cout << val_to_string(L) << endl;
    // show(L);
  }

  /*
  累積和で高速化した DP → 「累積和」と言う手法を理解.

  dp[i] := 最初の i 個のサプリまで吸収する方法の数 (i 個目を吸収した段階で一旦区切る)
  dp[0] = 1
  dp[i] = 区間 [j, i) が「複数種類のサプリがない」という条件を満たすような j についての dp[j] の総和
  */
  dp[0] = 1;
  sum[0] = 0;
  sum[1] = 1;
  int i = 1;
  for (i = 1; i <= N; ++i) {
    dp[i] = (sum[i] - sum[L[i]] + MOD) % MOD; // DP
    // debug
    // cout << "dp[" << i << "] " << dp[i] << " = sum[" << i << "] (" << sum[i] << ") - sum[" <<
    // L[i]
    //      << "] (" << sum[L[i]] << ")" << endl;

    sum[i + 1] = (sum[i] + dp[i]) % MOD; // 累積和
    // debug
    // cout << "sum[" << (i + 1) << "] " << sum[i + 1] << " = sum[" << i << "] (" << sum[i]
    //      << ") + dp[" << i << "] (" << dp[i] << ")" << endl
    //      << endl;
  }

  // debug
  // cout << val_to_string(L) << endl;
  // show(slice(L, 0, i));
  // cout << val_to_string(sum) << endl;
  // show(slice(sum, 0, i));
  // cout << val_to_string(dp) << endl;
  // show(slice(dp, 0, i));

  cout << dp[N] << endl;
}