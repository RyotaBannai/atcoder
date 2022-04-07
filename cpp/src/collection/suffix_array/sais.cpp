/*
Not Implemented Yet.

接尾辞配列（Suffix Array）
Suffix Array Induced Sort

参考
https://naoyat.hatenablog.jp/entry/construct-suffix-array-and-lcp-in-linear-time
https://gist.github.com/Kejuntrap/2c776012e5907b200efc438227a8fa76
*/
#include <algorithm>
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

  void createSuffixArray()
  {
    vector<int> rank(N + 1), tmp(N + 1);
    for (int i = 0; i <= N; ++i) {
      SA[i] = i;
      rank[i] = i < N ? (int)S[i] : -1;
    }

    // sais impl come here.
  }

public:
  SuffixArray(string s)
  {
    S = s;
    N = s.length();
    SA = vector<int>(N + 1, 0);
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
      int cmp = S.compare(index, t.size(), t);
      if (cmp < 0) {
        l = mid;
      }
      else {
        r = mid;
      }
    }
    return S.compare(SA[r], t.size(), t) == 0;
  }
};

auto main() -> int
{
  string s;
  cin >> s;
  SuffixArray sa{s};
  int N;
  cin >> N;
  while (N--) {
    string t;
    cin >> t;
    cout << sa.contains(t) << endl;
  }
}