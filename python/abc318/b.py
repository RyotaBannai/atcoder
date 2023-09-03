n = int(input())
xs = [list(map(int, input().split())) for _ in range(n)]

s = {}
for a, b, c, d in xs:
    for x in range(a, b):
        for y in range(c, d):
            s[(x, y)] = 1

print(len(s))
