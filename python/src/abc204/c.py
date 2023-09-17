"""https://atcoder.jp/contests/abc204/tasks/abc204_c.

各頂点から行ける子の頂点の数を調べる. 
#dfs
"""
import sys

sys.setrecursionlimit(10**9)

n, m = list(map(int, input().strip().split(" ")))
dat = [list(map(int, input().strip().split(" "))) for _ in range(m)]

ad: list[list[int]] = [[] for _ in range(n)]
for ab in dat:
    a, b = ab
    a -= 1
    b -= 1
    ad[a].append(b)


def fn(par: int, used: list[bool]) -> int:
    used[par] = True
    tmp = 1
    for u in ad[par]:
        if not used[u]:
            tmp += fn(u, used)
    return tmp


ans = 0
for i in range(n):
    used = [False] * n
    ans += fn(i, used)
print(ans)
