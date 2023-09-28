import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class Solution {

	public static void main(String[] args) throws IOException {
		FileReader fileReader = new FileReader("../input.txt");
		BufferedReader bufferedReader = new BufferedReader(fileReader);

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
		int sum = 0;

		for (String line : input) {
			char[] chars = line.toCharArray();

			int opponent = chars[0] - 'A';
			int player = chars[2] - 'X';

			// Game Result
			if ((opponent + 1) % 3 == player) {
				// Player won
				sum += 6;
			} else if (opponent == player) {
				// Tie
				sum += 3;
			}

			// Player Move
			sum += player + 1;
		}

		return sum;
	}

	public static int part_2(List<String> input) {
		int sum = 0;

		for (String line : input) {
			char[] chars = line.toCharArray();
			int opponent = chars[0] - 'A';
			int result = chars[2] - 'X';

			if (result == 0) {
				sum += (opponent + 2) % 3 + 1;
			} else if (result == 1) {
				sum += opponent + 4; // 1 + 3 (draw)
			} else {
				sum += (opponent + 1) % 3 + 7; // 1 + 6 (win)
			}
		}

		return sum;
	}
}
