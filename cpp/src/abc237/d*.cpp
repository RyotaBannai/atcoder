/* @cpg_dirspec d
LR insertion

.前から検討して埒があかない時は後方から実装を検討
.n 回時に、n-1 をベースに処理を決める場合は、後方から考えてみるとよい

deque ver.

LRRLR
1 2 4 5 3 0

はじめ、A=(0) です。
s1​ が L なので、A=(1,0) となります。
s2​ が R なので、A=(1,2,0) となります。
s3​ が R なので、A=(1,2,3,0) となります。
s4​ が L なので、A=(1,2,4,3,0) となります。
s5​ が R なので、A=(1,2,4,5,3,0) となります。

後方から実装を検討：
初めは [5]
R だから L [4,5]
L だから R [4,5,3]
R だから L [2,4,5,3]
R だから L [1,2,4,5,3]
L だから R [1,2,4,5,3,0]

 */
#include <algorithm>
#include <deque>
#include <iostream>
#include <string>
using namespace std;

auto main() -> int
{
  int N;
  cin >> N;
  string s;
  cin >> s;
  deque<int> deq;
  deq.push_back(N);
  for (int i = N - 1; i >= 0; i--) {
    if (s[i] == 'L') {
      deq.push_back(i);
    }
    else {
      deq.push_front(i);
    }
  }

  for_each(deq.begin(), deq.end(), [](int x) { cout << x << " "; });
  cout << endl;
}