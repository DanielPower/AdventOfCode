import math
total = 0
with open("../masses.txt", "r") as file:
    for line in file.readlines():
        mass = int(line[:-1])
        total += math.floor(mass/3)-2
    print(total)
