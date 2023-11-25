import math
total = 0


def required_fuel(mass):
    fuel = math.floor(mass/3)-2
    if fuel > 0:
        return fuel + required_fuel(fuel)
    else:
        return 0

with open("../masses.txt", "r") as file:
    masses = [int(i) for i in file.read().split()]
    for mass in masses:
        total += required_fuel(mass)

print(total)
    
