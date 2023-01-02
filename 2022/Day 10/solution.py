with open("input.txt") as file:
    strength_sum = 0
    cycle_breakpoints = [20,60,100,140,180,220]

    crt = [['.' for _ in range(40)] for _ in range(6)]
    
    cycle = 1
    x = 1

    def tick_cycle(change=0):
        global strength_sum
        global cycle
        global x
        if cycle < 240:
            pos = (cycle - 1) % 40
            paint = abs(pos - x) <= 1
            crt[cycle // 40][pos] = '#' if paint else ' '
        cycle += 1
        x += change
        if cycle in cycle_breakpoints:
            strength_sum += x * cycle


    for line in file.readlines():
        if line == "noop\n":
            tick_cycle()
        else:
            [_,a] = line.split(" ")
            V = int(a)
            tick_cycle()
            tick_cycle(change=V)
    print(f"Part 1: Sum of Signal Strengths is {strength_sum}")

    for row in crt:
        print("".join(row))
