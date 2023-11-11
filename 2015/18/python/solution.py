

def load_file(file):
    with open(file) as f:
        lines = f.readlines()
        grid = []
        for line in lines:
            row = []
            for c in line:
                row.append(c == '#')
            grid.append(row)
        return grid


def part_1(state):
    for _ in range(100):
        new_state = []
        for i in range(100):
            row = []
            for j in range(100):
                count = 0
                if i > 0 and state[i - 1][j]:
                    count += 1
                if j > 0 and state[i][j - 1]:
                    count += 1
                if i > 0 and j > 0 and state[i - 1][j - 1]:
                    count += 1
                if i < 99 and state[i + 1][j]:
                    count += 1
                if j < 99 and state[i][j + 1]:
                    count += 1
                if i > 0 and j < 99 and state[i - 1][j + 1]:
                    count += 1
                if j > 0 and i < 99 and state[i + 1][j - 1]:
                    count += 1
                if i < 99 and j < 99 and state[i + 1][j + 1]:
                    count += 1
                if state[i][j]:
                    row.append(count == 2 or count == 3)
                else:
                    row.append(count == 3)
            new_state.append(row)
        state = new_state

    count = 0
    for row in state:
        for col in row:
            if col:
                count += 1
    return count


def part_2(state):
    for _ in range(100):
        new_state = []
        for i in range(100):
            row = []
            for j in range(100):
                count = 0
                if i > 0 and state[i - 1][j]:
                    count += 1
                if j > 0 and state[i][j - 1]:
                    count += 1
                if i > 0 and j > 0 and state[i - 1][j - 1]:
                    count += 1
                if i < 99 and state[i + 1][j]:
                    count += 1
                if j < 99 and state[i][j + 1]:
                    count += 1
                if i > 0 and j < 99 and state[i - 1][j + 1]:
                    count += 1
                if j > 0 and i < 99 and state[i + 1][j - 1]:
                    count += 1
                if i < 99 and j < 99 and state[i + 1][j + 1]:
                    count += 1
                if state[i][j]:
                    row.append(count == 2 or count == 3)
                else:
                    row.append(count == 3)
            new_state.append(row)
        new_state[0][0] = True
        new_state[99][0] = True
        new_state[0][99] = True
        new_state[99][99] = True
        state = new_state

    count = 0
    for row in state:
        for col in row:
            if col:
                count += 1
    return count


if __name__ == '__main__':
    print(f'Part 1: {part_1(load_file("../input.txt"))}')
    print(f'Part 2: {part_2(load_file("../input.txt"))}')
