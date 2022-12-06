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
