def load_data(path):
    with open(path, "r") as f:
        for line in f:
            yield line


def preproc(line):
    data = line.replace(".", "").replace(",", "").split()
    main_bag = " ".join(list((data[0], data[1])))
    inner_bags = data[data.index("contain") + 1 :]
    cap_list = []
    for i, word in enumerate(inner_bags):
        if word == "no":
            cap_list.append({"null": 0})
            break
        if word == "bag" or word == "bags":
            number = inner_bags[i - 3]
            bag_type = " ".join(list((inner_bags[i - 2], inner_bags[i - 1])))
            cap_list.append({bag_type: number})
    return main_bag, cap_list


def count_numbers(dict_bags, key="shiny gold"):
    count = 0
    for bag in dict_bags[key]:
        key_inner = list(bag.keys())[0]
        if key_inner == "null":
            return count
        else:
            count += int(list(bag.values())[0])
            count += count_numbers(dict_bags, list(bag.keys())[0]) * int(
                list(bag.values())[0]
            )
    return count


def count_chosen_bag(dict_bags, bag_key):
    count = 0
    for bag in dict_bags[bag_key]:
        key_inner = list(bag.keys())[0]
        if key_inner == "null":
            return count
        else:
            if key_inner == "shiny gold":

                count += 1
            count += count_chosen_bag(dict_bags, key_inner)
    return count


dict_bags = {preproc(line)[0]: preproc(line)[1] for line in load_data("data")}

ct = 0
for key in dict_bags:
    if count_chosen_bag(dict_bags, key) > 0:
        ct += 1

print("Part 1: ", ct)
print("Part 2: ", count_numbers(dict_bags, "shiny gold"))
