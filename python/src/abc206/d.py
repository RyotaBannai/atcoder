"""https://atcoder.jp/contests/abc206/tasks/abc206_d.

#disjoint_set #union_find

- scipy 公式
https://docs.scipy.org/doc/scipy/reference/generated/scipy.cluster.hierarchy.DisjointSet.html
- union find 自前
https://kunassy.com/unionfind-atcoder-python/#toc1
"""

from scipy.cluster.hierarchy import DisjointSet


def rl() -> list[int]:
    return list(map(int, input().strip().split(" ")))


n, s = rl()[0], rl()
ds = DisjointSet(s)
[ds.merge(*ij) for ij in zip(s, s[::-1])]
ans = sum(len(vls) - 1 for vls in ds.subsets())
print(ans)
