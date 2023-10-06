import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Iterator;
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
			String first_half = line.substring(0, line.length() / 2);
			String second_half = line.substring(line.length() / 2);

			for (int c : first_half.chars().toArray()) {
				if (second_half.contains("" + ((char) c))) {
					if (c >= 'a') {
						sum += c - 'a' + 1;
					} else {
						sum += c - 'A' + 27;
					}
					break;
				}
			}
		}
		return sum;
	}

	public static int part_2(List<String> input) {

		int sum = 0;

		Iterator<String> iterator = input.iterator();

		while (iterator.hasNext()) {
			String a = iterator.next(), b = iterator.next(), c = iterator.next();
			List<Integer> a_b = new ArrayList<>();

			for (int ch : a.chars().toArray()) {
				if (b.contains("" + ((char) ch))) {
					a_b.add(ch);
				}
			}

			for (int ch : c.chars().toArray()) {
				if (a_b.contains(ch)) {
					if (ch >= 'a') {
						sum += ch - 'a' + 1;
					} else {
						sum += ch - 'A' + 27;
					}
					break;
				}
			}

		}

		return sum;
	}
}
