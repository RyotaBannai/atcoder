/*
区間スケジューリング問題:
特定の区間が決められた予定を実行できる最大値を求める.
制限: 同じ区間に実行できるものは一つのみ

上手くいかない例:
.開始時間が一番早い区間を選ぶことを繰り返す（反例: 選んだ最初の区間が長い場合）
.最も短い区間を選ぶことを繰り返す（反例: 区間が短いが複数の区間にまたがる場合）
.その区間を選んだ時に、選べなくなる区間数が最も少ない区間から選ぶことを繰り返す
（反例: 選んだ区間と組み合わせられる他の区間が、他を選んだ場合よりも少ない場合）

成功例:
.終了時間が最も早い区間を選ぶことを繰り返す


inputs:
5
1 3
2 5
4 7
6 9
8 10

3
*/

#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;

const int MAX = 100000;
int N;
vector<pair<int, int>> itv; // 区間

auto main() -> int
{
  itv.reserve(MAX);
  int N, s, e;
  cin >> N;
  // 成功例のように、終了時間を基にしたいため、first に終了時間をセット
  lp(i, N) cin >> s >> e, itv.emplace_back(e, s);
  // 終了時間が早い区間から取り出せるようにしておく
  sort(itv.begin(), itv.begin() + N);

  int ans = 0, preve = 0; // preve は一つ前の区間の終了時間
  lp(i, N) if (preve < itv[i].second) ans++, preve = itv[i].first;
  cout << ans << endl;
}