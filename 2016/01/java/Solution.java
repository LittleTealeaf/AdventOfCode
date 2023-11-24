import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class Solution {

	public static void main(String[] args) throws IOException {
		FileReader fileReader = new FileReader("../input.txt");
		BufferedReader bufferedReader = new BufferedReader(fileReader);

		String str = "";

		int c;
		while ((c = bufferedReader.read()) != -1) {
			str += (char) c;
		}

		bufferedReader.close();

		String[] values = str.trim().split(", ");

		System.out.println("Part 1: " + part_1(values));
		System.out.println("Part 2: " + part_2(values));
	}

	static int part_1(String[] input) {
		int x = 0;
		int y = 0;
		int direction = 0;

		for (String line : input) {
			char[] chars = line.toCharArray();
			if (chars[0] == 'R') {
				direction = (direction + 1) % 4;
			} else {
				direction = (direction + 3) % 4;
			}

			int steps = Integer.parseInt(line.substring(1));

			if (direction == 0) {
				x += steps;
			} else if (direction == 1) {
				y -= steps;
			} else if (direction == 2) {
				x -= steps;
			} else {
				y += steps;
			}
		}

		return Math.abs(x) + Math.abs(y);
	}

	public static class Position {
		int x, y;

		public Position(int x, int y) {
			this.x = x;
			this.y = y;
		}

		@Override
		public boolean equals(Object obj) {
			if (obj instanceof Position) {
				Position other = (Position) obj;
				return other.x == this.x && other.y == this.y;
			}
			return false;
		}

	}

	static int part_2(String[] input) {
		int x = 0;
		int y = 0;
		int direction = 0;

		List<Position> visited = new ArrayList<>();

		for (String line : input) {
			char[] chars = line.toCharArray();

			if (chars[0] == 'R') {
				direction = (direction + 1) % 4;
			} else {
				direction = (direction + 3) % 4;
			}

			int steps = Integer.parseInt(line.substring(1));

			for (int i = 0; i < steps; i++) {

				Position pos = new Position(x, y);

				if (!visited.contains(pos)) {
					visited.add(pos);
				} else {
					return Math.abs(x) + Math.abs(y);
				}

				if (direction == 0) {
					x += 1;
				} else if (direction == 1) {
					y -= 1;
				} else if (direction == 2) {
					x -= 1;
				} else {
					y += 1;
				}
			}
		}
		return -1;
	}
}
