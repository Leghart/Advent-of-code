import functools


def load_data(path):
    with open(path, "r") as f:
        data = f.read().split("\n")
        return [int(int_data) for int_data in data]


def progress_counter(data):
    out = [data[i + 1] - data[i] for i in range(len(data) - 1)]
    num_1, num_3 = out.count(1), out.count(3)
    return num_1 * num_3


@functools.lru_cache()
def rek(pos):
    if pos == len(data) - 1:
        return 1
    out = 0
    for i in range(pos + 1, len(data)):
        if data[i] - data[pos] <= 3:
            out += rek(i)
    return out


data = sorted(load_data("data"))
data.insert(0, 0)
data.append(max(data) + 3)
print(progress_counter(data))
print(rek(0))
