import os

with open("data.txt", "r") as f:
    data = f.read().splitlines()

tree = {}

for line in data:
    if line.startswith("$"):
        sign, cmd, *rest = line.split(" ")

        if cmd == "cd":
            path = rest[0]
            if path == "/":
                current_dir = path
            else:
                new_path = os.path.join(current_dir, path)
                current_dir = os.path.normpath(new_path)

            if current_dir not in tree:
                tree[current_dir] = {}
        if cmd == "ls":
            tmp_list = []

    else:
        p1, p2 = line.split(" ")
        if p1 == "dir":
            new_path = os.path.join(current_dir, p2)
            tmp_list.append(new_path)
        else:
            tmp_list.append((p1, p2))

        tree[current_dir] = tmp_list


def get_files_sizes(files: list) -> int:
    return sum([int(file[0]) for file in files])


def get_sizes(dir_content: list):
    if all([isinstance(item, tuple) for item in dir_content]):
        return get_files_sizes(dir_content)

    tmp = []
    for file in dir_content:
        if isinstance(file, str):
            tmp.append(get_sizes(tree[file]))
        else:
            tmp.append(get_files_sizes([file]))

    return sum(tmp)


new_tree = {dir: get_sizes(files) for dir, files in tree.items()}

print("Part 1: ", sum([size for size in new_tree.values() if size <= 100000]))

max_size = 70_000_000
required_free_space = 30_000_000

sorted_values = sorted(new_tree.values(), reverse=True)
unused_space = max_size - sorted_values[0]


possible_dirs_to_remove = [
    size_to_remove
    for size_to_remove in sorted_values
    if unused_space + size_to_remove >= required_free_space
]

print("Part 2: ", min(possible_dirs_to_remove))
