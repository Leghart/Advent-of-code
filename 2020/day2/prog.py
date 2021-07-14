import numpy as np

correct = 0


def get_params(origin_data):
    a = origin_data.split(" ")
    limits = a[0]
    tmp = limits.split("-")
    key_tmp = a[1]
    fp = tmp[0]
    lp = tmp[1]
    key = key_tmp[0]
    data = a[2]

    return fp, lp, key, data


file = open("data", "r")
w = file.read()
file.close()
a = w.split("\n")
all_data = np.array(a)

iter = 1
for row in all_data:
    if iter == 1000:
        break
    fp, lp, key, data = get_params(row)
    tmp = 0
    if data[int(fp) - 1] == str(key):
        tmp += 1
    if data[int(lp) - 1] == str(key):
        tmp += 1
    if tmp == 1:
        correct += 1
    iter += 1
print(correct)
