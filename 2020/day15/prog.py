import time

data = [int(i) for i in "5,2,8,16,18,0,1".split(",")]


def run(size):
    spoken = {number: turn + 1 for turn, number in enumerate(data)}
    next_number = 0

    for turn in range(len(data) + 1, size):
        if next_number not in spoken:
            new_number = 0
            spoken[next_number] = turn

        else:
            new_number = turn - spoken[next_number]
            spoken[next_number] = turn

        next_number = new_number
    return next_number


start = time.perf_counter()
print(run(size=30000000))
stop = time.perf_counter()
print("time: ", stop - start)

# # old version
# def run1(size):
#     for i in range(len(data), size):
#         if data[i - 1] not in data[: i - 2]:
#             data.append(0)
#         else:
#             last = len(data) - list(reversed(data)).index(data[i - 1]) - 1
#             prev = last - list(reversed(data[:last])).index(data[i - 1]) - 1
#             data.append(last - prev)
#         print(len(data))
#     return data[-1]
# print(run1(30000000))
