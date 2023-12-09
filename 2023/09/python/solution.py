with open("../input.txt") as file:
    data = []
    for item in file.readlines():
        data.append([int(s) for s in item.replace("\n", "").split(" ")])


def part_1_find_next(line: list[int]) -> int:
    diffs = []
    last = None
    all_zero = True
    for item in line:
        if item != 0:
            all_zero = False
        if last != None:
            diffs.append(item - last)
        last = item
    if all_zero:
        return 0

    if last == None:
        raise ValueError("Error")

    return last + part_1_find_next(diffs)


def part_2_find_next(line: list[int]) -> int:
    diffs = []
    last = None
    first = None
    all_zero = True
    for item in line:
        if item != 0:
            all_zero = False
        if last != None:
            diffs.append(item - last)
        else:
            first = item
        last = item
    if all_zero:
        return 0

    if last == None or first == None:
        raise ValueError("Error")

    return first - part_2_find_next(diffs)


if __name__ == "__main__":
    part_1 = 0
    part_2 = 0

    for item in data:
        part_1 += part_1_find_next(item)
        part_2 += part_2_find_next(item)

    print(f"Part 1: {part_1}")
    print(f"Part 2: {part_2}")
