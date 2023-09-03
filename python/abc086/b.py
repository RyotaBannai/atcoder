# -*- coding: utf-8 -*-
import math

a, b = map(int, input().split())
ins = int(str(a) + str(b))
myInt = math.sqrt(ins)
print("{}".format("Yes" if isinstance(myInt, int) else "Yes" if myInt.is_integer() else "No"))
