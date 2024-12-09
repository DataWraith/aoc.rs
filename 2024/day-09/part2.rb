input = ARGF.read.strip
input = input.split("")

switch = true
r = []
c = 0

input.each do |d|
  r << [c] * d.to_i if switch
  r << ['.'] * d.to_i unless switch
  c += 1 if switch
  switch = !switch
end

result = []

until r.empty? do
  x = r.shift

  if x.first != "."
    result << x
    next
  end

  found = false

  i = r.length
  r.reverse.each do |k|
    i -= 1
    next if k.nil?
    next if k.empty?
    next if k[0] == '.'

    if k.length == x.length
      result << k
      r[i] = ['.'] * x.length
      found = true
      break
    end

    if k.length < x.length
      diff = x.length - k.length
      result << k
      r[i] = ['.'] * k.length
      r.unshift ["."] * diff
      found = true
      break
    end
  end

  if !found
    result << x
  end
end

sum = 0
result.flatten.each_with_index do |d, i|
  next if d == '.'
  sum += d * i
end

p sum

