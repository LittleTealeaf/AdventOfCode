


int part1(IEnumerable<string> input)
{

    int score = 0;

    foreach (string line in input)
    {
        char[] line_chars = line.ToCharArray();

        int opponent = line_chars[0] - 'A';
        int player = line_chars[2] - 'X';

        if ((opponent + 1) % 3 == player)
        {
            score += 6;
        }
        else if (opponent == player)
        {
            score += 3;
        }


        score += player + 1;
    }

    return score;
}

int part2(IEnumerable<string> input)
{

    int score = 0;

    foreach (string line in input)
    {
        char[] line_chars = line.ToCharArray();

        int opponent = line_chars[0] - 'A';
        int result = line_chars[2] - 'X';

        if (result == 0)
        {
            score += (opponent + 2) % 3 + 1;
        }
        else if (result == 1)
        {
            score += opponent + 4;
        }
        else
        {
            score += (opponent + 1) % 3 + 7;
        }

    }


    return score;
}





Console.WriteLine("Part 1: " + part1(File.ReadLines("../input.txt")));
Console.WriteLine("Part 2: " + part2(File.ReadLines("../input.txt")));
