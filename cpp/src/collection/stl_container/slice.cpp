#include <vector>
using namespace std;

/**
 * @brief return sliced vector from original vector
 *
 * @tparam T
 * @param v          original vector
 * @param start      0-indexed
 * @param end        size + 1
 * @return vector<T> new sliced vector
 */
template <typename T> auto slice(vector<T> v, int start, int end = -1) -> vector<T>
{
  if (end == -1) {
    end = v.size();
  }
  auto it = v.begin();
  advance(it, end);
  if (it <= v.end()) { // end が範囲内
    return vector<T>(v.begin() + start, v.begin() + end);
  }
  else { // end が範囲外の場合は、end
    return vector<T>(v.begin() + start, v.end());
  }
}