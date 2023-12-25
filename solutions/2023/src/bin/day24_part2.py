from z3 import *
from pathlib import Path
import pprint

# Load the input file
with open(f"{Path.home()}/.aoc/2023/24.inp") as f:
    lines = f.readlines()

# pprint.pprint(lines);
# input = """19, 13, 30 @ -2,  1, -2
# 18, 19, 22 @ -1, -1, -2
# 20, 25, 34 @ -2, -2, -4
# 12, 31, 28 @ -1, -2, -1
# 20, 19, 15 @  1, -5, -3
# """
# lines = input.splitlines()

x_0, y_0, z_0, vx__0, vy_0, vz_0, t = Reals('x__0 y__0 z__0 vx__0 vy__0 vz__0 t')

# Create all the time variables and put them into an array prior to the loop.
# Doing this in the loop didn't seem to work
ts = Reals(" ".join([f"t{i+1}" for i in range(0, len(lines))]))
i = 0
s = Solver()
for line in lines:
    parts = line.replace(" ", "").split("@")
    p = [int(x) for x in parts[0].strip().split(",")]
    v = [int(x) for x in parts[1].strip().split(",")]

    s.add(x_0 + vx__0*ts[i] == p[0] + v[0]*ts[i])
    s.add(y_0 + vy_0*ts[i] == p[1] + v[1]*ts[i])
    s.add(z_0 + vz_0*ts[i] == p[2] + v[2]*ts[i])
    i += 1

s.check()
m = s.model()
sum = 0

# Just running solve truncates some of the output, so we instead
# create a solver and access the position coordinates manually
for k in m:
     # print('%s=%s' % (k, m[k]))
     sum += int(f"{m[k]}")

# Add the position coordinates together
print(f"Answer={sum}")
