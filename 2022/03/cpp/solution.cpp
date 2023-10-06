#include <cstdio>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int part_1(std::vector<std::string> &lines) {
  int sum = 0;

  for (int i = 0; i < lines.size(); i++) {
    std::string line = lines[i];

    for (int j = 0; j < line.size() / 2; j++) {
      bool found = false;
      for (int k = line.size() / 2; k < line.size(); k++) {
        if (line[j] == line[k]) {
          found = true;
          break;
        }
      }
      if (found) {
        if (line[j] >= 'a') {
          sum += line[j] - 'a' + 1;
        } else {
          sum += line[j] - 'A' + 27;
        }
        break;
      }
    }
  }

  return sum;
}

int part_2(std::vector<std::string> &lines) {
  int sum = 0;

  for (int i = 0; i < lines.size(); i += 3) {
    for (int a = 0; a < lines[i].size(); a++) {
      bool found = false;
      for (int b = 0; b < lines[i + 1].size(); b++) {
        if (lines[i][a] == lines[i + 1][b]) {
          for (int c = 0; c < lines[i + 2].size(); c++) {
            if (lines[i][a] == lines[i + 2][c]) {
              found = true;
              break;
            }
          }
        }

        if (found) {
          break;
        }
      }

      if (found) {
        if (lines[i][a] >= 'a') {
          sum += lines[i][a] - 'a' + 1;
        } else {
          sum += lines[i][a] - 'A' + 27;
        }
        break;
      }
    }
  }

  return sum;
}

int main() {

  std::vector<std::string> lines;
  std::ifstream file("../input.txt");

  std::string line;
  while (std::getline(file, line)) {
    lines.push_back(line);
  }

	file.close();

  std::cout << "Part 1: " << part_1(lines) << std::endl;
  std::cout << "Part 2: " << part_2(lines) << std::endl;

  return 0;
}
