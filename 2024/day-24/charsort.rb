x = ARGF.read.strip.lines

puts x.sort_by { |line| 
  k = line.chars.sort
  [k.length, k]
}.join("\n")
