/* @cpg_dirspec d
Sequence Query
https://atcoder.jp/contests/abc241/tasks/abc241_d

TLE
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

template <class T, class Compare = std::less<T>> struct sorted_vector {
  vector<T> V;
  Compare cmp;
  typedef typename vector<T>::iterator iterator;
  typedef typename vector<T>::const_iterator const_iterator;
  iterator begin() { return V.begin(); }
  iterator end() { return V.end(); }
  const_iterator begin() const { return V.begin(); }
  const_iterator end() const { return V.end(); }
  sorted_vector(const Compare &c = Compare()) : V(), cmp(c) {}
  template <class InputIterator>
  sorted_vector(InputIterator first, InputIterator last, const Compare &c = Compare())
      : V(first, last), cmp(c)
  {
    std::sort(begin(), end(), cmp);
  }
  iterator insert(const T &t)
  {
    iterator i = lower_bound(begin(), end(), t, cmp);
    // if (i == end() || cmp(t, *i))
    V.insert(i, t);
    return i;
  }
  const_iterator find(const T &t) const
  {
    const_iterator i = lower_bound(begin(), end(), t, cmp);
    return i == end() || cmp(t, *i) ? end() : i;
  }
};

auto main() -> int
{
  ll N;
  sorted_vector<ll> s;
  cin >> N;

  while (N--) {
    int q;
    cin >> q;
    if (q == 1) {
      ll x;
      cin >> x;
      s.insert(x);
    }
    else if (q == 2) {
      ll x, k;
      cin >> x >> k;
      auto it = upper_bound(s.begin(), s.end(), x); // x より小さい
      bool over = false;
      while (k--) {
        if (it == s.begin()) {
          over = true;
          break;
        }
        else {
          it--;
        }
      }
      cout << (over ? -1 : *it) << endl;
      // if (distance(s.begin(), it) < k) {
      //   cout << -1 << endl;
      // }
      // else {
      //   advance(it, -k);
      //   cout << *it << endl;
      // }
      /** distance は ok */
    }
    else if (q == 3) {
      ll x, k;
      cin >> x >> k;
      auto it = lower_bound(s.begin(), s.end(), x); // x より大きい
      if (distance(it, s.end()) < k) {
        cout << -1 << endl;
      }
      else {
        advance(it, k - 1);
        cout << *it << endl;
      }
    }
  }
}