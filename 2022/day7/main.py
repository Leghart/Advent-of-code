with open("data.txt", "r") as f:
    data = f.read().splitlines()

b = []
for i in data:
    a = i.split(" ")
    if a[0] == "dir":
        b.append(a[1])
print("dirs len: ", len(b))

tree = {}
current_dir = "/"
for line in data:
    if line.startswith("$"):
        line_command = line.split(" ")
        if len(line_command) == 3 and line_command[2].isalpha():
            current_dir = line_command[2]
            tree[current_dir] = {}

        elif line_command[1] == "ls":
            tmp_list = []

    else:
        p1, p2 = line.split(" ")
        if p1 == "dir":
            tmp_list.append(p2)
        else:
            tmp_list.append((p1, p2))

        tree[current_dir] = tmp_list


# def get_files_sizes(files: list) -> int:
#     return sum([int(file[0]) for file in files])


# def get_sizes(dir_content: list):
#     if all([isinstance(item, tuple) for item in dir_content]):
#         return get_files_sizes(dir_content)

#     tmp = []
#     for file in dir_content:
#         if isinstance(file, str):
#             tmp.append(get_sizes(tree[file]))
#         else:
#             tmp.append(get_files_sizes([file]))

#     return sum(tmp)


# new_tree = {}
# for dir, files in tree.items():
#     new_tree[dir] = get_sizes(files)

# for a, b in new_tree.items():
#     print(a, b)


# print("Part 1: ", sum([size for size in new_tree.values() if size <= 100000]))
