"""https://atcoder.jp/contests/abc320/tasks/abc320_b."""
s: str = input()
ans = 1
for i in range(0, len(s)):
    tmp = 1
    j = 1
    while i - j >= 0 and i + j < len(s):
        pre, nex = i - j, i + j
        if s[pre] != s[nex]:
            break
        tmp += 2
        if ans < tmp:
            ans = tmp
        j += 1

for i in range(0, len(s)):
    tmp = 0
    j = 1
    while i - j + 1 >= 0 and i + j < len(s):
        pre, nex = i - j + 1, i + j
        if s[pre] != s[nex]:
            break
        tmp += 2
        if ans < tmp:
            ans = tmp
        j += 1

print(ans)
