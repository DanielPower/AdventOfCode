import math
import sys
total = 0
for line in sys.stdin:
    mass = int(line.strip())
    total += math.floor(mass/3)-2
print(total)
