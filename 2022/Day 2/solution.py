with open("input.txt") as file:
    score = 0
    for line in file.readlines():
        a = ord(line[0]) - ord('A')
        b = ord(line[2]) - ord('X')
        score += b + 1
        
        if (b - a) % 3 == 1:
            score += 6
        elif b == a:
            score += 3
    print(score)

with open("input.txt") as file:
    score = 0

    for line in file.readlines():
        a = ord(line[0]) - ord('A')
        b = ord(line[2]) - ord('X')
        score += b * 3
        
        if b == 0:
            score += (a - 1) % 3 + 1
        elif b == 1:
            score += a + 1
        elif b == 2:
            score += (a + 1) % 3 + 1

    print(score)
