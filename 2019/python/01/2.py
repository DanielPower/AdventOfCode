import math
import sys
total = 0

def required_fuel(mass):
    fuel = math.floor(mass/3)-2
    if fuel > 0:
        return fuel + required_fuel(fuel)
    else:
        return 0

for line in sys.stdin:
    mass = int(line.strip())
    total += required_fuel(mass)

print(total)
    
