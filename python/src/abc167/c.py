"""https://atcoder.jp/contests/abc167/tasks/abc167_c.

#bit全探索
"""


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


n, m, x = readline()
dat = [readline() for _ in range(n)]
ans = 10**9
for i in range(1 << n):
    # 各パターン
    C = 0
    skill = [0] * m
    for j in range(n):
        if i & 1 == 1:
            C += dat[j][0]
            for k in range(m):
                skill[k] += dat[j][k + 1]

        i >>= 1

    if all(y >= x for y in skill):
        ans = min(ans, C)

if ans == 10**9:
    print(-1)
else:
    print(ans)
