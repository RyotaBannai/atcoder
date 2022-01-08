/* @cpg_dirspec d
Prefix K-th Max

n 番目に大きい → 要素を追加したときに、一番小さい要素と比較して常に最後尾が n 番目となるように管理
pointer で最後尾を追跡してもいいし、
priority_queue のようなので top を最後尾として管理しても良い
（比較して、追加した要素より小さければ pop）

https://cpprefjp.github.io/reference/set/set/begin.html
Or
https://cpprefjp.github.io/reference/queue/priority_queue.html
*/
#include <algorithm>
#include <iostream>
#include <set>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll n, k;
  cin >> n >> k;
  ll arr[n];
  for (ll i = 0; i < n; i++)
    cin >> arr[i];
  set<ll> s;
  for (ll i = 0; i < k; i++) {
    s.insert(arr[i]);
  }
  auto it = s.begin();
  cout << *it << endl;
  for (ll i = k; i < n; i++) {
    s.insert(arr[i]);
    if (arr[i] > *it)
      it++;
    cout << *it << endl;
  }
}