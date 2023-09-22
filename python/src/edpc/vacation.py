"""https://atcoder.jp/contests/dp/tasks/dp_c."""


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


N = readline()[0]
abcs = readlines(N)
dp: list[list[int]] = [[0 for _ in range(3)] for _ in range(N + 1)]

for i in range(N):
    for to in range(3):
        for fr in range(3):
            if to != fr:
                dp[i + 1][to] = max(dp[i + 1][to], dp[i][fr] + abcs[i][to])


print(max(dp[N]))
