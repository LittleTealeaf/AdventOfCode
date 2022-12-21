def decode_steps(lines: list[str]):
    direction = {
        "R": (1, 0),
        "L": (-1, 0),
        "U": (0, 1),
        "D": (0, -1)
    }
    value: list[tuple[tuple[int,int],int]] = []
    for line in lines:
        [a,b] = line.split(" ")
        value.append((direction[a],int(b)))
    return value

def simulate(moves: list[tuple[tuple[int,int],int]], size: int):
    positions: list[tuple[int,int]] = [(0, 0)]
    hx = 0
    hy = 0
    tx = 0
    ty = 0

    tail_moves = []
    for (dx,dy),step in moves:
        for i in range(step):
            hx += dx
            hy += dy
            if abs(hx - tx) > 1 or abs(hy - ty) > 1:
                tdx = 0
                tdy = 0
                if abs(hx - tx) > 0:
                    dx1 = abs(hx - tx) // (hx - tx)
                    tx += dx1
                    tdx = dx1
                if abs(hy - ty) > 0:
                    dy1 = abs(hy - ty) // (hy - ty)
                    ty += dy1
                    tdy = dy1
                tail_moves.append(((tdx,tdy),1))
                if (tx,ty) not in positions:
                    positions.append((tx,ty))
    if size > 1:
        return simulate(tail_moves,size-1)
    else:
        return len(positions)

with open("input.txt") as file:
    steps = decode_steps(file.readlines())
    part_1 = simulate(steps,1)
    print(f"Part 1: {part_1} unique positions")
    part_2 = simulate(steps,9)
    print(f"Part 2: {part_2} unique positions")
