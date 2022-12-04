with open("data.txt", "r") as f:
    common = 0
    common2 = 0
    for line in f.read().splitlines():
        first, second = line.split(",")
        first_num1, first_num2 = first.split("-")
        second_num1, second_num2 = second.split("-")
        range1 = range(int(first_num1), int(first_num2) + 1)
        range2 = range(int(second_num1), int(second_num2) + 1)
        intersection = set(range1) & set(range2)

        if len(intersection) in (len(range1), len(range2)):
            common += 1

        if intersection:
            common2 += 1

    print("Part 1: ", common)
    print("Part 2: ", common2)
