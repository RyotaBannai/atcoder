"""https://atcoder.jp/contests/abc320/tasks/abc320_b."""
s: str = input()
n = len(s)
ans = 0
for i in range(n):
    for size in range(n + 1):
        tmp = s[i : i + size]
        if tmp == tmp[::-1]:
            ans = max(ans, len(tmp))
print(ans)
