valid = 0

for i in range(193651, 649729+1):
    string_value = str(i)
    is_decreasing = True
    for j in range(len(string_value)-1):
        if string_value[j] > string_value[j+1]:
            is_decreasing = False

    good = False
    duplicates = 0
    prev_char = ''
    for char in string_value:
        if char == prev_char:
            duplicates += 1
        else:
            if duplicates == 1:
                break
            duplicates = 0
        prev_char = char
    if duplicates == 1:
        good = True

    if is_decreasing and good:
        valid += 1
    elif is_decreasing:
        print(string_value)

print(valid)
