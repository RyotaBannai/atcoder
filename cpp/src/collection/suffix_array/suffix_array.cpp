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
#include <vector>
#define ll long long
using namespace std;

class SuffixArray {
private:
  string S;
  int N;
  void CreateSuffixArray()
  {
    for (int i = 0; i <= N; ++i;) {
      SA[i] = i;
    }
  }

public:
  vector<int> SA; // suffix array の index
  SuffixArray(string s)
  {
    this.S = s;
    this.N = s.length;
    this.SA.assign(N + 1, 0);
    CreateSuffixArray();
  }
};

auto main() -> int { SuffixArray sa{"abcabc"}; }