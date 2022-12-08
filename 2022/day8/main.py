with open("data.txt", "r") as f:
    matrix = [[int(i) for i in line] for line in f.read().splitlines()]


def is_visible(row: int, col: int, matrix: list[list[int]]) -> bool:
    # edges
    if row in (0, len(matrix) - 1) or col in (0, len(matrix) - 1):
        return True

    for top in range(row):
        if matrix[row][col] <= matrix[top][col]:
            break
    else:
        return True

    for bottom in range(len(matrix) - 1, row, -1):
        if matrix[row][col] <= matrix[bottom][col]:
            break
    else:
        return True

    for left in range(col):
        if matrix[row][col] <= matrix[row][left]:
            break
    else:
        return True

    for right in range(len(matrix[0]) - 1, col, -1):
        if matrix[row][col] <= matrix[row][right]:
            break
    else:
        return True

    return False


def tree_score(row: int, col: int, matrix: list[list[int]]) -> int:
    def visible_trees(row: int, col: int, matrix: list[list[int]]) -> list[int]:
        # edges
        if row in (0, len(matrix) - 1) or col in (0, len(matrix) - 1):
            return [0]

        for top in range(1, row + 1):
            if matrix[row][col] <= matrix[row - top][col]:
                break

        for bottom in range(1, len(matrix) - row):
            if matrix[row][col] <= matrix[row + bottom][col]:
                break

        for left in range(1, col + 1):
            if matrix[row][col] <= matrix[row][col - left]:
                break

        for right in range(1, len(matrix[0]) - col):
            if matrix[row][col] <= matrix[row][col + right]:
                break

        return [top, bottom, left, right]

    scores = visible_trees(row, col, matrix)

    result = 1
    for i in scores:
        result *= i

    return result


visible = []
scores = []
for row in range(len(matrix)):
    for col in range(len(matrix[0])):
        visible.append(is_visible(row, col, matrix))
        scores.append(tree_score(row, col, matrix))

print("Part 1: ", sum(visible))
print("Part 2: ", max(scores))
