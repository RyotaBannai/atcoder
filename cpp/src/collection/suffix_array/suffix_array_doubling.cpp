/*
接尾辞配列（Suffix Array）
Doubling を用いて Suffix Array をソート

参考
https://webbibouroku.com/Blog/Article/suffix-array#outline__1
*/
#include <iostream>
#include <set>
#include <string>
#include <vector>
#define ll long long
using namespace std;

class SuffixArray {
private:
  string S;
  int N;
  vector<int> SA; // suffix array の index

  struct CompareSA {
    int n, k;
    const vector<int> &rank;
    CompareSA(int n, int k, const vector<int> &rank) : n(n), k(k), rank(rank) {}
    auto operator()(int i, int j) -> bool
    {
      // 前の k 分で順位が違っていれば、それ以降の K+1 順位は前後しないため、k^2 ではそのまま返す
      if (rank[i] != rank[j]) {
        return rank[i] < rank[j];
      }
      else {
        int rank_ik = (i + k <= n ? rank[i + k] : -1);
        int rank_jk = (j + k <= n ? rank[j + k] : -1);
        return rank_ik < rank_jk;
      }
    }
  };

  void createSuffixArray()
  {
    vector<int> rank(N + 1), tmp(N + 1);
    for (int i = 0; i <= N; ++i) {
      // SA は単なる部分文字列の index の位置
      SA[i] = i;
      // rank=1 のソート前の順位は、ascii 文字コードで順位付 から文字は先頭に来るように -1
      rank[i] = i < N ? (int)S[i] : -1;
    }

    for (int k = 1; k <= N; k *= 2) {
      // cout << "k:" << k << endl;
      /*
      x: k <= N の範囲でソートするだけで k^2 までのソート順が保証される. のはなぜ
      o: 初期の文字コードを入れた時点で初めのソートが行われているため、k=1 の時は 2
      文字目までのソート k=2 で 4 文字目、k=3 で 8 文字目、k=4 で 16 文字目
      */
      CompareSA csa(N, k, rank);
      sort(SA.begin(), SA.end(), csa);

      tmp[SA[0]] = 0;
      for (int i = 1; i <= N; ++i) {
        tmp[SA[i]] = tmp[SA[i - 1]];
        if (csa(SA[i - 1], SA[i])) {
          tmp[SA[i]]++;
        }
      }
      // 次の回（k^2）用に rank を更新
      for (int i = 0; i <= N; ++i) {
        rank[i] = tmp[i];
      }
    }
    // debug
    // for (auto x : SA) {
    //   cout << S.substr(x) << endl;
    // }
    // cout << endl;
  }

public:
  SuffixArray(string s)
  {
    S = s;
    N = s.length();
    SA = vector<int>(N + 1, 0); // 空も使う
    createSuffixArray();
  }

  // 部分文字列にTが含まれるかどうかを判定（二分探索）
  auto contains(string t) -> bool
  {
    int l = 0;
    int r = N;
    while (r - l > 1) {
      int mid = l + (r - l) / 2;
      int index = SA[mid];
      int cmp = S.compare(index, t.size(), t); // S.compare は S 自身が対象に対してどうか.
      if (cmp < 0) {
        l = mid;
      }
      else {
        r = mid;
      }
    }
    // cout << S.substr(SA[r]) << endl;
    return S.compare(SA[r], t.size(), t) == 0;
  }
};

auto main() -> int
{
  SuffixArray sa{"abracadabra"};
  cout << (sa.contains("abracadabra") ? "Yes" : "No") << endl;
}