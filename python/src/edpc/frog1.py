"""https://atcoder.jp/contests/dp/tasks/dp_a."""


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


N = readline()[0]
hs = readline()
DEFAULT = 10**16
dp = [DEFAULT] * N
dp[0] = 0
for i in range(N):
    for nex in [i + 1, i + 2]:
        if nex < N:
            dp[nex] = min(dp[nex], dp[i] + abs(hs[i] - hs[nex]))


print(dp[N - 1])
