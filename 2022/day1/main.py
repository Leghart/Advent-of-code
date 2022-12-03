with open("data.txt", "r") as f:
    data = [line.strip() for line in f.readlines()]

elf = 1
sum_calories = 0
elves_calories = {}

for calories in data:
    if calories == "":
        elf += 1
        sum_calories = 0
    else:
        sum_calories += int(calories)

    elves_calories[elf] = sum_calories

print("Part 1: ", max(elves_calories.values()))
print("Part 2: ", sum(sorted(elves_calories.values())[-3:]))
