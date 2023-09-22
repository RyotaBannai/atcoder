"""https://atcoder.jp/contests/dp/tasks/dp_b."""


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


N, K = readline()
hs = readline()
DEFAULT = 10**10
dp = [DEFAULT] * N
dp[0] = 0
for i in range(N):
    for d in range(K):
        nex = d + i + 1
        if nex < N:
            dp[nex] = min(dp[nex], dp[i] + abs(hs[i] - hs[nex]))
        else:
            break


print(dp[N - 1])
