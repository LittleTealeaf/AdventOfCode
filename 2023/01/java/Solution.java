import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Solution {

	public static void main(String[] args) throws IOException {
		FileReader fileReader = new FileReader("../input.txt");
		BufferedReader bufferedReader = new BufferedReader(fileReader);
		List<String> lines = bufferedReader.lines().collect(Collectors.toList());
		bufferedReader.close();

		System.out.println("Part 1: " + part_1(lines));
		System.out.println("Part 2: " + part_2(lines));
	}

	public static int part_1(List<String> lines) {

		int sum = 0;

		for (String line : lines) {
			Integer first = null;
			Integer last = null;

			for (int i = 0; i < line.length(); i++) {
				char ch = line.charAt(i);

				if (ch >= '0' && ch <= '9') {
					if (first == null) {
						first = (int) ch - '0';
					}

					last = (int) ch - '0';
				}
			}

			sum += first * 10 + last;
		}

		return sum;
	}

	public static int part_2(List<String> lines) {

		int sum = 0;

		String[] words = { "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" };

		for (String line : lines) {
			// First number
			Integer first = null;
			for (int i = 0; i < line.length(); i++) {
				char ch = line.charAt(i);
				if (ch >= '0' && ch <= '9') {
					first = ch - '0';
					break;
				}

				// See if it's the start of any number

				for (int j = 0; j < words.length; j++) {
					String word = words[j];

					// Skip this word if there's not enough space
					if (word.length() > line.length() - i) {
						continue;
					}

					boolean match = true;

					// Check match
					for (int k = 0; k < word.length(); k++) {
						// i is the offset
						if (line.charAt(i + k) != word.charAt(k)) {
							match = false;
							break;
						}
					}

					if (match) {
						first = j + 1;
					}
				}

				if (first != null) {
					break;
				}
			}

			Integer last = null;
			for (int i = line.length() - 1; i >= 0; i--) {
				char ch = line.charAt(i);
				if (ch >= '0' && ch <= '9') {
					last = ch - '0';
					break;
				}

				// See if it's the start of any number

				for (int j = 0; j < words.length; j++) {
					String word = words[j];

					// Skip this word if there's not enough space
					if (word.length() > line.length() - i) {
						continue;
					}

					boolean match = true;

					// Check match
					for (int k = 0; k < word.length(); k++) {
						// i is the offset
						if (line.charAt(i + k) != word.charAt(k)) {
							match = false;
							break;
						}
					}

					if (match) {
						last = j + 1;
					}
				}

				if (last != null) {
					break;
				}
			}

			sum += first * 10 + last;
		}
		return sum;
	}

}
