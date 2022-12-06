with open("input.txt") as file:
    count = 0
    for line in file.readlines():
        ranges = [[int(i) for i in elf.split("-")] for elf in line.split(",")]
        a_a = ranges[0][0]
        a_b = ranges[0][1]
        b_a = ranges[1][0]
        b_b = ranges[1][1]

        if (a_a <= b_a and a_b >= b_b) or (b_a <= a_a and b_b >= a_b):
            count += 1
    print(count)
        

