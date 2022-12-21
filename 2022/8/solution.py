with open("input.txt") as file:
    grid = [[int(i) for i in line[:-1]] for line in file.readlines()]
    visibility = [[False for _ in row] for row in grid]
    height = len(grid)
    width = len(grid[0])

    for y in range(height):
        i = -1
        j = -1
        for x in range(width):
            xp = width - x - 1
            if grid[y][x] > i:
                i = grid[y][x]
                visibility[y][x] = True
            if grid[y][xp] > j:
                j = grid[y][xp]
                visibility[y][xp] = True
    for x in range(width):
        i = -1
        j = -1
        for y in range(height):
            yp = height - y - 1
            if grid[y][x] > i:
                i = grid[y][x]
                visibility[y][x] = True
            if grid[yp][x] > j:
                j = grid[yp][x]
                visibility[yp][x] = True
    count = 0
    for row in visibility:
        for col in row:
            if col:
                count += 1
    print(f"Part 1: Count is {count} ")

    scenic_score_max = 0

    for x in range(1, width-1):
        for y in range(1, height - 1):
            tree_height = grid[y][x]
            score = 1
            for dx, dy in [(-1, 0), (1, 0), (0, 1), (0, -1)]:
                for steps in range(1,max(height, width)):
                    ix = x + dx * steps
                    iy = y + dy * steps
                    if ix == 0 or iy == 0 or ix == width - 1 or iy == height - 1 or grid[iy][ix] >= tree_height:
                        score *= steps
                        break
            if score > scenic_score_max:
                scenic_score_max = score

    print(f"Part 2: Max Scenic Score is {scenic_score_max}")
