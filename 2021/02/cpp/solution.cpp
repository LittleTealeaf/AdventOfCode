

#include <fstream>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int part_1(vector<string> input) {
  int horizontal = 0, depth = 0;

  for (string line : input) {
    if (line[0] == 'f') {
      horizontal += stoi(line.substr(8));
    } else if (line[0] == 'd') {
      depth += stoi(line.substr(5));
    } else {
      depth -= stoi(line.substr(3));
    }
  }

  return horizontal * depth;
}

int part_2(vector<string> input) {
  int horizontal = 0, depth = 0, aim = 0;

  for (string line : input) {
    if (line[0] == 'f') {
      int x = stoi(line.substr(8));
      horizontal += x;
      depth += aim * x;
    } else if (line[0] == 'd') {
      aim += stoi(line.substr(5));
    } else {
      aim -= stoi(line.substr(3));
    }
  }

  return horizontal * depth;
}

int main() {

  vector<string> lines;
  ifstream file("../input.txt");

  string line;
  while (getline(file, line)) {
    lines.push_back(line);
  }
  file.close();

  cout << "Part 1: " << part_1(lines) << endl;
  cout << "Part 2: " << part_2(lines) << endl;

  return 0;
}
