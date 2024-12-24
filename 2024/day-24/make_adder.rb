def carry(depth)
  idx_str = depth.to_s.rjust(2, '0')

  if depth == 0
    return "(x#{idx_str} AND y#{idx_str})"
  end

  "(x#{idx_str} AND y#{idx_str}) OR ((x#{idx_str} XOR y#{idx_str}) AND #{carry(depth-1)}"
end

def result(depth)
  idx_str = depth.to_s.rjust(2, '0')

  if depth == 0
    return "(x#{idx_str} XOR y#{idx_str})"
  end

  "(x#{idx_str} XOR y#{idx_str}) OR #{carry(depth-1)}"
end

xs = []

(0..44).each do |i|
  xs << result(i)
end

xs.sort_by { |x| [x.length, x] }.each do |x|
  puts x
end
  

