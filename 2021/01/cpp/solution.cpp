#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int part_1(vector<string> input) {
  int prev = stoi(input[0]);
  int count = 0;
  for (int i = 1; i < input.size(); i++) {
    int curr = stoi(input[i]);
    if (curr > prev) {
      count += 1;
    }
    prev = curr;
  }
  return count;
}

int part_2(vector<string> input) {
  int buffer[3] = {stoi(input[0]), stoi(input[1]), stoi(input[2])};
  int count = 0;
  for (int i = 3; i < input.size(); i++) {
    int curr = stoi(input[i]);
    int sum = buffer[0] + buffer[1] + buffer[2];
    int new_sum = buffer[1] + buffer[2] + curr;
    if (new_sum > sum) {
      count += 1;
    }
    buffer[0] = buffer[1];
    buffer[1] = buffer[2];
    buffer[2] = curr;
  }
  return count;
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
