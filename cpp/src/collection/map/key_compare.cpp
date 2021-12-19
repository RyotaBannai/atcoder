/*
key_compare : Map key comparison
map_compare : Map value comparison

https://stackoverflow.com/questions/8473009/how-to-efficiently-compare-two-maps-of-strings-in-c-only-for-a-subset-of-the-k

std::equal:
https://cpprefjp.github.io/reference/algorithm/equal.html
*/
#include <algorithm>
#include <iostream>
#include <map>
#include <string>
#include <utility>

template <typename Map> auto map_compare(Map const &lhs, Map const &rhs) -> bool
{
  // No predicate needed because there is operator== for pairs already.
  return lhs.size() == rhs.size() && std::equal(lhs.begin(), lhs.end(), rhs.begin());
}

template <typename Map> auto key_compare(Map const &lhs, Map const &rhs) -> bool
{
  auto pred = [](auto a, auto b) { return a.first == b.first; };
  return lhs.size() == rhs.size() && std::equal(lhs.begin(), lhs.end(), rhs.begin(), pred);
}

auto main() -> int
{
  using namespace std;

  map<string, string> a, b;

  a["Foo"] = "0";
  a["Bar"] = "1";
  a["Frob"] = "2";

  b["Foo"] = "0";
  b["Bar"] = "1";
  b["Frob"] = "2";

  cout << "a == b? " << map_compare(a, b) << " (should be 1)\n";
  b["Foo"] = "1";
  cout << "a == b? " << map_compare(a, b) << " (should be 0)\n";

  map<string, string> c;
  cout << "a == c? " << map_compare(a, c) << " (should be 0)\n";

  cout << "keys of a == b? " << key_compare(a, b) << " (should be 1)\n";
}