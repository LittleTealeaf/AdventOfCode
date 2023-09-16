

def read_input():
    with open('../../../inputs/2022/01/input.txt') as file:
        return file.readlines()


def part_1(input_data):
    max_value = 0
    current = 0
    for line in input_data:
        if line == '\n':
            if current > max_value:
                max_value = current
            current = 0
        else:
            current += int(line.replace('\n', ''))
    return max_value


def part_2(input_data):
    elves = []
    current = 0
    for line in input_data:
        if line == '\n':
            elves.append(current)
            current = 0
        else:
            current += int(line.replace('\n', ''))
    elves.sort()
    return elves[-1] + elves[-2] + elves[-3]


if __name__ == '__main__':
    input_data = read_input()
    print("Part 1:", part_1(input_data))
    print("Part 2:", part_2(input_data))
