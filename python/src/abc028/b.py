"""."""
from collections import Counter

c = Counter(input().strip())
print(" ".join(map(lambda a: str(c.get(a, 0)), "ABCDEF")))
