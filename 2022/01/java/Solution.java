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
		int current = 0, max = 0;

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
		int[] top = new int[3];
		int current = 0;

		for (String line : input) {
			if (line.equals("")) {
				if (current > top[0]) {
					top[2] = top[1];
					top[1] = top[0];
					top[0] = current;
				} else if (current > top[1]) {
					top[2] = top[1];
					top[1] = current;
				} else if (current > top[2]) {
					top[2] = current;
				}
				current = 0;
			} else {
				current += Integer.parseInt(line);
			}
		}
		return top[0] + top[1] + top[2];
	}
}
