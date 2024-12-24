input = ARGF.read

x = input.lines.map(&:strip)

equivalences = {}

x.each do |line|
  left, op, right, nix, result = line.split

  left, right = [left, right].sort

  equivalences[result] = ["(#{left} #{op} #{right})", "(#{right} #{op} #{left})"]
end

15.times do
equivalences.each do |result, lr|
  left, right = lr
  input.gsub!(result, left)
  input.gsub!(result, right)
end
end

l = input.lines.sort_by { |x| [x.length, x] }
k = []

l.each do |line|
  left, right = line.split(" -> ")
  next if left.gsub("(", "").gsub(")", "").strip == right.gsub("(", "").gsub(")", "").strip
  k << line
end

puts k.sort.join("\n")

