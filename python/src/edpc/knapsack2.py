"""https://atcoder.jp/contests/dp/tasks/dp_e.

制限を満たすように上限を設定.
"""


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


N, W = readline()
wv = readlines(N)
max_value = 10**5
DEFAULT = 10**10
dp: list[list[int]] = [[DEFAULT for _ in range(max_value + 1)] for _ in range(N + 1)]
dp[0][0] = 0  # 先頭のみ

for i in range(N):
    w, v = wv[i]
    for j in range(max_value - v + 1):
        dp[i + 1][j] = min(dp[i + 1][j], dp[i][j])
        nv = j + v  # 配る
        nw = dp[i][j] + w
        if nw <= W:
            dp[i + 1][nv] = min(dp[i + 1][nv], nw)

ans = 0
for v in range(max_value + 1):
    if dp[N][v] < DEFAULT:
        ans = v

print(ans)
