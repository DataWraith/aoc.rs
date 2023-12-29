# From https://www.youtube.com/watch?v=guOyA7Ijqgk

import sympy

hailstones = [tuple(map(int, line.replace("@", ",").split(","))) for line in open("input.txt")]

xr, yr, zr, vxr, vyr, vzr = sympy.symbols("xr, yr, zr, vxr, vyr, vzr")

equations = []

for (sx, sy, sz, vx, vy, vz) in hailstones[:4]:
    equations.append((xr - sx) * (vy - vyr) - (yr - sy) * (vx - vxr))
    equations.append((yr - sy) * (vz - vzr) - (zr - sz) * (vy - vyr))

print(sympy.solve(equations))
