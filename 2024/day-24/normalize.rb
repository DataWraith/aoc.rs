x = File.read("input copy.txt").strip.lines
y = []

x.each do |line|
  op, result = line.strip.split(" -> ")
  left, operator, right = op.split

  a, b = [left, right].sort
  a = [a, operator, b].join(" ")

  y << [a, result].join(" -> ")
end

y.sort_by! { |x| [x.length, x] }
y.each do |line|
puts line
end
