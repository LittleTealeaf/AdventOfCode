with open("input.txt") as file:
    positions = [(0,0)]

    hx = 0
    hy = 0
    tx = 0
    ty = 0

    direction = {
        "R": (1, 0),
        "L": (-1, 0),
        "U": (0, 1),
        "D": (0, -1)
    }

    for line in file.readlines():
        [a, b] = line.split(" ")
        (dx,dy) = direction[a]
        steps = int(b)
        for i in range(steps):
            hx += dx
            hy += dy
            if abs(hx - tx) > 1 or abs(hy - ty) > 1:
                if abs(hx - tx) > 0:
                    tx += abs(hx - tx) / (hx - tx)
                if abs(hy - ty) > 0:
                    ty += abs(hy - ty) / (hy - ty)
                if (tx,ty) not in positions:
                    positions.append((tx,ty))
    print(f"Part 1: The tail visited {len(positions)} unique positions")
