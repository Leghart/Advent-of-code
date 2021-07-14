def load_data(path):
    with open(path, "r") as f:
        data = f.read().split("\n")
        return list(map(int, data))


def progress_counter(data):
    data.insert(0, 0)
    data.append(max(data) + 3)
    out = [data[i + 1] - data[i] for i in range(len(data) - 1)]
    _1, _3 = out.count(1), out.count(3)
    return _1 * _3


data = sorted(load_data("data"))
print(progress_counter(data))
