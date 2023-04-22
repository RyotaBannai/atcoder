_ = read_line
xs = read_line.split.map(&.to_i64)
count = 0
while xs.all? { |x| x % 2 == 0 }
  count += 1
  xs = xs.map { |x| x / 2 }
end
puts count
