/* @cpg_dirspec d
LR insertion

two vector ver.
.先頭から処理を検討するバージョン
.n-1 をベースに n 回を検討する場合は、先に n-1 の処理をする代わりに、n 回目に n-1
回目の処理をどうすれば良いかという点で実装を検討する
.今回は n 回目自身が動くのではなく、n 回目の指示をもとに n-1 の数値を動かすことを考える

n 回目に n-1 目の数字を以下のルールで処理をする
n 回目が L ならば、right に push_back（n が Left 側に来る）
n 回目が R ならば、left に push_back（n が Right 側に来る）

R の要素の並びを反転して出力
 */
#include <algorithm>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

auto main() -> int
{
  int N;
  cin >> N;
  string s;
  cin >> s;
  vector<int> L, R;
  for (int i = 0; i < N; i++) {
    if (s[i] == 'L') {
      R.push_back(i);
    }
    else {
      L.push_back(i);
    }
  }

  auto fmt = [](int x) { cout << x << " "; };
  for_each(L.begin(), L.end(), fmt);
  fmt(N);
  for_each(R.rbegin(), R.rend(), fmt);
  cout << endl;
}