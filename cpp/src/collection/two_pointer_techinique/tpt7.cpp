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

template <class T> void show(vector<T> var)
{
  for (auto x : var)
    cout << x << " ";
  cout << endl;
}

const int MOD = 1000000007;
vector<int> dp(110000);
vector<int> sum(110000);

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
    // cout << val_to_string(fnum) << endl;
    // show(fnum);

    while (left < right && fnum[f[right]] > 1) {
      --fnum[f[left]];
      ++left;
    }
    L[right + 1] = left;
    // cout << val_to_string(L) << endl;
    // show(L);
  }

  /*
  dp[i] := 最初の i 個のサプリまで吸収する方法の数 (i 個目を吸収した段階で一旦区切る)
  dp[0] = 1
  dp[i] = 区間 [j, i) が「複数種類のサプリがない」という条件を満たすような j についての dp[j] の総和
  */
  /* 累積和で高速化した DP */
  dp[0] = 1;
  sum[0] = 0;
  sum[1] = 1;
  for (int i = 1; i <= N; ++i) {
    dp[i] = (sum[i] - sum[L[i]] + MOD) % MOD; // DP
    cout << "dp[" << i << "] " << dp[i] << " sum[" << i << "] " << sum[i] << " sum[" << L[i] << "] "
         << sum[L[i]] << endl;

    sum[i + 1] = (sum[i] + dp[i]) % MOD; // 累積
    cout << "sum[" << (i + 1) << "] " << sum[i + 1] << endl;
  }
  cout << val_to_string(sum) << endl;
  show(sum);
  cout << val_to_string(dp) << endl;
  show(dp);
  /*
  dp[1] 1
  sum[2] 2
  dp[2] 2
  sum[3] 4
  dp[3] 3
  sum[4] 7
  dp[4] 5
  sum[5] 12
  dp[5] 5
  sum[6] 17
  */

  cout << dp[N] << endl;
}