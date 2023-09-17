from collections import defaultdict

n, x, y = list(map(int, input().split(" ")))
x -= 1
y -= 1
ret: defaultdict = defaultdict(int)
for i in range(n):
    for j in range(i + 1, n):
        if i == j:
            continue
        k = 10**6
        k = min(k, j - i)
        k = min(k, abs(x - i) + abs(y - j) + 1)
        ret[k] += 1


for i in range(1, n):
    ans = ret.get(i)
    p = 0 if ans is None else ans
    print(p)
