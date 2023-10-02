#include <cstdio>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int part_1(std::vector<std::string> &lines) {
  int score = 0;

  for (int i = 0; i < lines.size(); i++) {
    std::string line = lines[i];

    int opponent = line[0] - 'A';
    int player = line[2] - 'X';

    // Game Result
    if ((opponent + 1) % 3 == player) {
      // Player won
      score += 6;
    } else if (opponent == player) {
      // Tie
      score += 3;
    }

    // Player Move
    score += player + 1;
  }

  return score;
}

int part_2(std::vector<std::string> &lines) {
  int score = 0;

  for (int i = 0; i < lines.size(); i++) {
    std::string line = lines[i];

    int opponent = line[0] - 'A';
    int result = line[2] - 'X';

    if (result == 0) {
      score += (opponent + 2) % 3 + 1;
    } else if (result == 1) {
      score += opponent + 4; // 1 + 3 (draw)
    } else {
      score += (opponent + 1) % 3 + 7; // 1 + 6 (win)
    }
  }

  return score;
}

int main() {
  std::vector<std::string> lines;
  std::ifstream file("../input.txt");

  std::string line;
  while (std::getline(file, line)) {
    lines.push_back(line);
  }

  std::cout << "Part 1: " << part_1(lines) << std::endl;
  std::cout << "Part 2: " << part_2(lines) << std::endl;

  return 0;
}
