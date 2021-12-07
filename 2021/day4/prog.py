N = 5

with open("data", "r") as f:
    numbers = f.readline()
    data = f.readlines()

numbers = [int(i) for i in numbers.split(",")]
data = [i.strip() for i in data]
data = [i for i in data if i]
data = [[int(j) for j in i.split(" ") if j] for i in data]
boards = {i: data[i * N : N * (i + 1)] for i in range(int(len(data) / N))}
indexes = {i: [] for i in boards.keys()}


def check_win(board, indexes):
    from collections import Counter

    if len(indexes[board]) < 5:
        return None

    xs = Counter([i[0] for i in indexes[board]]).most_common()
    ys = Counter([i[1] for i in indexes[board]]).most_common()
    x_row, x_amount = max(xs, key=lambda x: x[1])
    y_row, y_amount = max(ys, key=lambda x: x[1])

    if x_amount == N:
        return {
            "x": x_row,
            "y": None,
            "board": board,
            "last": indexes[board][-1],
        }
    elif y_amount == N:
        return {
            "x": None,
            "y": y_row,
            "board": board,
            "last": indexes[board][-1],
        }
    else:
        return None


def run():
    for num in numbers:
        for board in boards:
            for id_row, row in enumerate(boards[board]):
                try:
                    id_col = row.index(num)
                except ValueError:
                    pass
                else:
                    pos = (id_row, id_col)
                    indexes[board].append(pos)
            possible_pos = check_win(board, indexes)
            if possible_pos is not None:
                return possible_pos


possible_pos = run()
row, col, board, last = (
    possible_pos["x"],
    possible_pos["y"],
    possible_pos["board"],
    possible_pos["last"],
)

winning_row = (
    boards[board][row] if row is not None else [row[col] for row in boards[board]]
)
sum_all = sum([sum(row) for row in boards[board]])
sum_marked = sum([boards[board][x][y] for x, y in indexes[board]])
sum_rest = sum_all - sum_marked
last_marked = boards[board][last[0]][last[1]]
print(sum_rest * last_marked)
