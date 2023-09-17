"""https://atcoder.jp/contests/abc320/tasks/abc320_a."""
xs = list(map(int, input().split()))
a, b = xs[0], xs[1]
print(pow(a, b) + pow(b, a))
