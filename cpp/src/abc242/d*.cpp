/* @cpg_dirspec d
ABC Transform

https://atcoder.jp/contests/abc242/tasks/abc242_d

 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  /**
   * A → BC, B → CA, C → AB と同時に置き換える
   * i.g.
   * t=0 abc を同時に置き換えた時
   * t=1 bccaab
   * t=2 caababbcbcca
   *
   *   0    1    2
   * 0 a    b    c
   * 1 bc   ca   ab
   * 2 caab abbc bcca
   *
   *
   * t>0, k>0 の時
   *
   * t=1,k=2(k は 0index,&& 偶数) の時
   * k=2 は c（この時点では、c であるかは判明していない）
   * t-1=1-1=0,k/2=2/2=1 一つ前の文字列の 1 番目を指す（b）（偶数. 3/2
   * でも同じ一つ前の生成先を参照できる）← 初めの g(f(...), ...) 部分
   * この時 t=0 なので、S[k] を返すため、S[1]==b 再起処理の末端は b なる
   * b を受け取った一つ上の関数は、g(b, k%2+1=2%2+1=1) これは、int(b)+1 より c となる.
   *
   *
   * t=2,k=1(k は 0index,&& 奇数) の時
   * k=1 は a（この時点では、a であるかは判明していない）
   * t-1=2-1=1,k/2=1/2=0 一つ前の文字列の 0 番目を指す（b)
   * k=0 より、文字列の t 番目（この時は t=1）と分かるため、abc の t=1 先の文字 b が末端になる
   * g(b,k%2+1=1%2+1=2) これは int(b)+2 より a となる.
   *
   *
   * ポイント：
   * まずは文字列を作れないから別の方法を模索
   * 目的の文字を見つけるために、n-1の文字をどうすれば特定できるか、（目的の文字/2の余を捨てた整数のindex）
   * n-1 から n になったときにどう操作すればいいかを考える (i.g. 奇数なら 2 、偶数なら、1)
   * 再帰で解けそう → 末尾の処理を考える（減らしている変数が 0 の時の処理を考える）
   *
   * 参考
   * https://atcoder.jp/contests/abc242/editorial/3520
   * https://qiita.com/u2dayo/items/465c9299755ed20ee4d3#d%E5%95%8F%E9%A1%8Cabc-transform
   */

  string S;
  cin >> S;
  int Q;
  cin >> Q;

  auto g = [](char s, ll add) { return char('A' + (s - 'A' + add) % 3); };

  std::function<char(ll, ll)> f = [&](ll t, ll k) {
    if (t == 0)
      return S[k];
    if (k == 0)
      return g(S[0], t);
    return g(f(t - 1, k / 2), k % 2 + 1);
  };

  while (Q--) {
    ll t, k;
    cin >> t >> k;
    cout << f(t, k - 1) << endl; // k は 1 以上だが、0-index として扱う
  }
}