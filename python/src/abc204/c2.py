"""https://atcoder.jp/contests/abc204/tasks/abc204_c.

各頂点から行ける子の頂点の数を調べる. 
#dfs
"""

from collections import deque

n, m = list(map(int, input().strip().split(" ")))
dat = [list(map(int, input().strip().split(" "))) for _ in range(m)]

ad: list[list[int]] = [[] for _ in range(n)]
for ab in dat:
    a, b = ab
    a -= 1
    b -= 1
    ad[a].append(b)


ans = 0
for i in range(n):
    used = [False] * n
    q = deque([i])
    count = 0
    while len(q) > 0:
        u = q.pop()
        if used[u]:
            continue
        used[u] = True
        count += 1
        for y in ad[u]:
            if not used[y]:
                q.append(y)
    ans += count
print(ans)
