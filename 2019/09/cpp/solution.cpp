#include <fstream>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int part_1(vector<int> inputs) {
  int sum = 0;
  for (int mass : inputs) {
    sum += mass / 3 - 2;
  }
  return sum;
}

int part_2(vector<int> inputs) {
  int sum = 0;
  for (int mass : inputs) {
    while (mass > 0) {
      mass /= 3;
      mass -= 2;
      if (mass > 0) {
        sum += mass;
      }
    }
  }
  return sum;
}

int main() {
  vector<int> inputs;
  ifstream file("../input.txt");

  string line;
  while (getline(file, line)) {
    inputs.push_back(stoi(line));
  }
  file.close();

  cout << "Part 1: " << part_1(inputs) << endl;
  cout << "Part 2: " << part_2(inputs) << endl;

  return 0;
}
