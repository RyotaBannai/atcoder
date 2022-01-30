/* @cpg_dirspec d
LR insertion

linked list ver.
.最も直感的な、n-1 回目の位置を追跡する手法
.n-1 の位置について、n 回目の処理を行う

linked list では：
.insert はポインタの位置に新しい要素を挿入し、ポインタの位置はもとの要素を指す
つまり、もとの要素の前に新しい要素を追加する（左側に追加する処理）
.ポインタはもとの位置にあるから、指示が R であれば、現在のポインタの位置に挿入すれば（右側に追加）
指示が　L であれば、ポインタを一つ戻してあげれば、自分の n-1
に挿入できるから左側に追加する処理となる
 */
#include <algorithm>
#include <iostream>
#include <list>
#include <string>
using namespace std;

auto main() -> int
{
  int N;
  cin >> N;
  string s;
  cin >> s;
  list<int> list;
  list.push_back(0);
  auto it = list.end();
  for (int i = 0; i < N; i++) {
    if (s[i] == 'L') {
      list.insert(--it, i + 1);
    }
    else {
      list.insert(it, i + 1);
    }
  }

  auto fmt = [](int x) { cout << x << " "; };
  for_each(list.begin(), list.end(), fmt);
  cout << endl;
}