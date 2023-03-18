
with open("input.txt") as file:
    lines = file.readlines()
    breakpt = lines.index("\n")
    input_state = lines[0:breakpt-1]

    state = []
    for i in range((len(input_state[-1]) + 1) // 4):
        state.append([])
    for line in input_state[0:len(input_state)]:
        if(line.startswith("    ")):
            line = "[_] " + line[4:]
        line = line.replace("    "," [_]")
        for i in range(len(state)):
            value = line[i * 4 + 1]
            if(value != '_'):
                state[i].insert(0,line[i*4+1])

    instructions = lines[breakpt+1:]
    for step in instructions:
        values = step.replace("\n","").split(" ")
        count = int(values[1])
        start = int(values[3]) - 1
        finish = int(values[5]) - 1

        for i in range(count):
            state[finish].append(state[start].pop(-1))
    print("Part 1 Solution: ", "".join([col[-1] for col in state]))

with open("input.txt") as file:
    lines = file.readlines()
    breakpt = lines.index("\n")
    input_state = lines[0:breakpt-1]

    state = []
    for i in range((len(input_state[-1]) + 1) // 4):
        state.append([])
    for line in input_state[0:len(input_state)]:
        if(line.startswith("    ")):
            line = "[_] " + line[4:]
        line = line.replace("    "," [_]")
        for i in range(len(state)):
            value = line[i * 4 + 1]
            if(value != '_'):
                state[i].insert(0,line[i*4+1])

    instructions = lines[breakpt+1:]
    for step in instructions:
        values = step.replace("\n","").split(" ")
        count = int(values[1])
        start = int(values[3]) - 1
        finish = int(values[5]) - 1

        values = [state[start].pop(-1) for _ in range(count)]
        for i in range(count):
            state[finish].append(values.pop(-1))

    print("Part 2 Solution: ","".join([col[-1] for col in state]))

#                [M]     [V]     [L]
#[G]             [V] [C] [G]     [D]
#[J]             [Q] [W] [Z] [C] [J]
#[W]         [W] [G] [V] [D] [G] [C]
#[R]     [G] [N] [B] [D] [C] [M] [W]
#[F] [M] [H] [C] [S] [T] [N] [N] [N]
#[T] [W] [N] [R] [F] [R] [B] [J] [P]
#[Z] [G] [J] [J] [W] [S] [H] [S] [G]
# 1   2   3   4   5   6   7   8   9
