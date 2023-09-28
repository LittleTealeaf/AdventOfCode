#include <cstdio>
#include <fstream>
#include <iostream>
#include <ostream>
#include <string>
#include <vector>

int part_1(std::vector<std::string> &lines) {
  int max = 0;
  int current = 0;
  for (int i = 0; i < lines.size(); i++) {
    std::string line = lines[i];

    if (line == "") {
      if (max < current) {
        max = current;
      }
      current = 0;
    } else {
      int number = std::stoi(line);
      current += number;
    }
  }

  return max;
}

int part_2(std::vector<std::string> &lines) {
  int *array = new int[3];
  int current = 0;
  for (int i = 0; i < lines.size(); i++) {
    std::string line = lines[i];

    if (line == "") {
      if (current > array[0]) {
        array[2] = array[1];
        array[1] = array[0];
        array[0] = current;
      } else if (current > array[1]) {
        array[2] = array[1];
        array[1] = current;
      } else if (current > array[2]) {
        array[2] = current;
      }
      current = 0;
    } else {
      int number = std::stoi(line);
      current += number;
    }
  }

  return array[0] + array[1] + array[2];
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
