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

s = r.flatten

fwd = 0
bwd = s.length - 1
bwd -= 1 while s[bwd] == '.'
r2 = []

while fwd <= bwd
  if s[fwd] == '.'
    r2 << s[bwd]
    fwd += 1
    bwd -= 1
    bwd -= 1 while s[bwd] == '.'
  else
    r2 << s[fwd]
    fwd += 1
  end
end

sum = 0

r2.each_with_index do |x, i|
  sum += x * i
end

p sum
