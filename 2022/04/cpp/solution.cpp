

#include <fstream>
#include <iostream>
#include <ostream>
#include <sstream>
#include <string>
#include <vector>

int part1(std::vector<std::string> input) {
  int count = 0;

  for (std::string line : input) {
    std::string buffer;
    std::vector<int> values;

    for (char c : line) {
      if (c == '-' || c == ',') {
        values.push_back(std::stoi(buffer));
        buffer.clear();
      } else {
        buffer.push_back(c);
      }
    }
    values.push_back(std::stoi(buffer));

    if (values[3] - values[2] > values[1] - values[0]) {
      int tmp = values[0];
      values[0] = values[2];
      values[2] = tmp;
      tmp = values[1];
      values[1] = values[3];
      values[3] = tmp;
    }

    if (values[0] <= values[2] && values[1] >= values[3]) {
      count += 1;
    }
  }

  return count;
}

int part2(std::vector<std::string> input) {
  int count = 0;

  for (std::string line : input) {
    std::string buffer;
    std::vector<int> values;

    for (char c : line) {
      if (c == '-' || c == ',') {
        values.push_back(std::stoi(buffer));
        buffer.clear();
      } else {
        buffer.push_back(c);
      }
    }
    values.push_back(std::stoi(buffer));

    if (values[0] > values[2]) {
      int tmp = values[0];
      values[0] = values[2];
      values[2] = tmp;
      tmp = values[1];
      values[1] = values[3];
      values[3] = tmp;
    }

    if (values[1] >= values[2]) {
      count += 1;
    }
  }

  return count;
}

int main() {
  std::vector<std::string> lines;
  std::ifstream file("../input.txt");

  std::string line;
  while (std::getline(file, line)) {
    lines.push_back(line);
  }

  file.close();

  std::cout << "Part 1: " << part1(lines) << std::endl;
  std::cout << "Part 2: " << part2(lines) << std::endl;
}
