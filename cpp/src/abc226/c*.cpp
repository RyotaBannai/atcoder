#include <iostream>
#include <queue>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

static const int MAX = 200000;
vector<vector<ll>> reqs;
vector<ll> T;

auto main() -> int
{
  reqs.reserve(MAX + 1);
  T.reserve(MAX + 1);

  int N, r;
  ll t;
  cin >> N;
  for (int i = 1; i <= N; i++) {
    cin >> T[i];
    cin >> r;
    for (int j = 0; j < r; j++) {
      cin >> t, reqs[i].push_back(t);
    }
  }

  // used := 術 B が必要な A に必要な C が B でも必要な場合でも A
  // のスキルセットに含まれている場合があるため、 術 A で取得する場合は２度修練を行わないよう弾く.
  bool used[N];
  for (int k = 1; k <= N; k++)
    used[k] = false;

  queue<int> q;
  q.push(N);
  used[N] = true;
  ll sum = 0;
  while (!q.empty()) {
    int u = q.front();
    q.pop();
    sum += T[u];
    for (int x : reqs[u]) {
      if (!used[x]) {
        q.push(x);
        used[x] = true;
      }
    }
  }

  cout << sum << endl;

  // for (int i = 1; i <= N; i++) {
  //   for (int x : reqs[i])
  //     cout << x;
  //   cout << endl;
  // }

  // for (int i = 1; i <= N; i++) {
  //   cout << T[i] << endl;
  // }
}