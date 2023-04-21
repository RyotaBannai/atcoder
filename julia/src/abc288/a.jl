_ = parse(Int64, readline())
s = readline()
# d = Dict{Char,Int64}() 
d = Dict() # 型推論できる
for k in ['o', '-', 'x']
  d[k] = 0
end
for i in eachindex(s)
  d[s[i]] += 1
end
if d['o'] >= 1 && d['x'] == 0
  println("Yes")
else
  println("No")
end