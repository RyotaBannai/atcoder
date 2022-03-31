/*


接尾辞配列（せつびじはいれつ, suffix array）:
文字列の接尾辞（開始位置を異にし終端位置を元の文字列と同じくする部分文字列）
の文字列中の開始位置を要素とする配列を、接尾辞に関して辞書順に並べ替えて得られる配列

→ 文字列を先頭 index 0 から length -1 の間で開始位置を決めて、それぞれを末尾まで subtar
した部分文字列を配列として、それらを辞書順で並び替えたもの.

→ 部分文字列に対して、二分探索することで部分文字列の検索が高速に行える　

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

public:
  SuffixArray(string s)
  {
    S = s;
    N = s.length();
    SA.assign(N + 1, 0); // 空も使う
    for (int i = 0; i <= N; ++i) {
      SA[i] = i;
    }
    // sort by alphabetically
    sort(SA.begin(), SA.end(), [this](int i, int j) { return S.substr(i) < S.substr(j); });

    // debug
    // for (auto x : SA) {
    //   cout << S.substr(x) << endl;
    // }
    // cout << endl;
  }

  // 部分文字列にTが含まれるかどうかを判定（二分探索）
  auto contains(string t) -> bool
  {
    int l = 0;
    int r = N;
    while (r - l > 1) {
      int mid = l + (r - l) / 2;
      int index = SA[mid];
      int cmp = t < S.substr(index); // S.compare(index, t.size(), t);

      // t の方が大きければ、最小を中央に更新　
      if (!cmp) { // cmp < 0 t の方が大きい場合 -> S.compare は S 自身が対象に対してどうか.
        l = mid;
      }
      else {
        r = mid;
      }
    }
    // cout << S.substr(SA[r]) << endl;
    // mid ではなく、辞書的により大きい r を index とした部分文字列を使う.
    return S.substr(SA[r]).find(t) != std::string::npos; // S.compare(SA[r], t.size(), t) == 0;
  }
};

auto main() -> int
{
  SuffixArray sa{"apobcabcmcsaidsfosijfa"};
  cout << (sa.contains("fo") ? "Yes" : "No") << endl;
}