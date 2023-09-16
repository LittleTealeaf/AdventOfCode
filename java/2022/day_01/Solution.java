import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class Solution {

	public static void main(String[] args) throws IOException {
		FileReader file = new FileReader("../../../inputs/2022/01/input.txt");
		BufferedReader bufferedReader = new BufferedReader(file);

		String line;
		List<String> input = new ArrayList<>();

		while ((line = bufferedReader.readLine()) != null) {
			input.add(line);
		}

		bufferedReader.close();

		System.out.println("Part 1: " + part_1(input));
		System.out.println("Part 2: " + part_2(input));

	}

	public static int part_1(List<String> input) {

		int max = 0;
		int current = 0;

		for (String line : input) {
			if (line.equals("")) {
				if (current > max) {
					max = current;
				}
				current = 0;
			} else {
				current += Integer.parseInt(line);
			}
		}

		return max;
	}

	public static int part_2(List<String> input) {
		int[] leaderboard = new int[3];
		int current = 0;

		for (String line : input) {
			if (line.equals("")) {
				if (current > leaderboard[0]) {
					leaderboard[2] = leaderboard[1];
					leaderboard[1] = leaderboard[0];
					leaderboard[0] = current;
				} else if (current > leaderboard[1]) {
					leaderboard[2] = leaderboard[1];
					leaderboard[1] = current;
				} else if (current > leaderboard[2]) {
					leaderboard[2] = current;
				}

				current = 0;
			} else {
				current += Integer.parseInt(line);
			}
		}

		return leaderboard[0] + leaderboard[1] + leaderboard[2];
	}
}
