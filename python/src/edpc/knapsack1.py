"""https://atcoder.jp/contests/dp/tasks/dp_d."""


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


N, W = readline()
wv = readlines(N)
dp: list[list[int]] = [[0 for _ in range(W + 1)] for _ in range(N + 1)]

for i in range(N):
    for w in range(W + 1):
        dp[i + 1][w] = max(dp[i + 1][w], dp[i][w])
        nw = w + wv[i][0]
        if nw <= W:
            dp[i + 1][nw] = max(dp[i + 1][nw], dp[i][w] + wv[i][1])


print(max(dp[N]))
