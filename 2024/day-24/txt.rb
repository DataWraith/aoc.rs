x = File.read("input copy.txt").strip.lines
y = File.read("gen2.txt").strip

2.times do
  x.each do |line|
    op, result = line.strip.split(" -> ")

    left, operator, right = op.split

    a = [left, operator, right].join(" ")
    b = [right, operator, left].join(" ")

    y.gsub!(result, a)
    y.gsub!(result, b)
  end
end

puts y

