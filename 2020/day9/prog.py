import copy
import itertools as it


def load_data(path):
    with open(path, "r") as f:
        data = f.read().split("\n")
        return list(map(int, data))


def check_weakness(data, wrong_ans, idx):
    sum_tmp = 0
    list_values = []
    for i, val in enumerate(data[idx:]):
        sum_tmp += val
        list_values.append(val)
        if sum_tmp == wrong_ans:
            min_val, max_val = min(list_values), max(list_values)
            return min_val + max_val, False
        elif sum_tmp > wrong_ans:
            return None, True


def check_valid(data, idx, window, val_to_sum):
    selected_data = data[idx : idx + window]  # noqa: E203
    ob = it.combinations(selected_data, 2)
    summed = [sum(i) for i in ob]
    if val_to_sum not in summed:
        return val_to_sum


data = load_data("data")
window = 25

for i in range(len(data) - window + 1):
    ans = check_valid(data, i, window, data[window + i])
    if ans is not None:
        break

still = True
for i in range(len(data)):
    if still:
        out, still = check_weakness(data, ans, i)
print(out)
