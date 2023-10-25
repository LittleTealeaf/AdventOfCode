// See https://aka.ms/new-console-template for more information



int part1(IEnumerable<string> lines)
{

    int max = 0;
    int current = 0;

    foreach (string line in lines)
    {
        int i;
        if (int.TryParse(line, out i))
        {
            current += i;
        }
        else
        {
            if (current > max)
            {
                max = current;
            }
            current = 0;
        }
    }

    return max;
}


int part2(IEnumerable<string> lines)
{

    int[] leaderboard = new int[3];

    int current = 0;

    foreach (string line in lines)
    {
        int i;
        if (int.TryParse(line, out i))
        {
            current += i;
        }
        else
        {

            if (current > leaderboard[0])
            {
                leaderboard[2] = leaderboard[1];
                leaderboard[1] = leaderboard[0];
                leaderboard[0] = current;
            }
            else if (current > leaderboard[1])
            {
                leaderboard[2] = leaderboard[1];
                leaderboard[1] = current;
            }
            else if (current > leaderboard[2])
            {
                leaderboard[2] = current;
            }
            current = 0;
        }
    }

    return leaderboard[0] + leaderboard[1] + leaderboard[2];
}


Console.WriteLine("Part 1: " + part1(File.ReadLines("../input.txt")));
Console.WriteLine("Part 2: " + part2(File.ReadLines("../input.txt")));
