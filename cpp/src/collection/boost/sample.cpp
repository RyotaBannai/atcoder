#include <boost/foreach.hpp>
#include <boost/range/adaptor/strided.hpp>
#include <boost/range/adaptor/transformed.hpp>
#include <boost/range/algorithm.hpp>
#include <boost/range/algorithm/copy.hpp>
#include <boost/range/irange.hpp>
#include <iostream>
#include <string>
#include <vector>

/*
https://qiita.com/Lily0727K/items/ab089a4974c614e7f861#boostmath-toolsbrent_find_minima
*/

using namespace std;

template <class T> void dump(vector<T> &v)
{
  BOOST_FOREACH (T x, v) {
    cout << x << " ";
  }
  cout << endl;
}

void test()
{
  vector<int> v{4, 3, 2, 1};
  dump(v);

  boost::sort(v);
  dump(v);

  exit(0);
}

void test2()
{
  boost::copy(boost::irange(0, 100) | boost::adaptors::strided(2) |
                  boost::adaptors::transformed([](int n) { return n * 2; }),
              std::ostream_iterator<int>(std::cout, " "));
}
auto main() -> int {}