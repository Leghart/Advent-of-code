import numpy as np

file = open("data", "r")
w = file.read()
file.close()
a = w.split()
a = np.array(a).astype(int)

tmp = []
for i in range(len(a)):
    for it in range(len(a)):
        for kk in range(len(a)):
            if a[it] + a[i] + a[kk] == 2020:
                tmp.append(a[it])
                tmp.append(a[i])
                tmp.append(a[kk])
roz = len(tmp)
roz = 3
tmp = tmp[: int(roz)]

out = tmp[0] * tmp[1] * tmp[2]
print(out)
