xs = list(map(int, input().split()))
n, m, p = xs[0], xs[1], xs[2]
if n < m:
    print(0)
else:
    n -= m
    print(1 + n // p)
