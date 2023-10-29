
#include <fstream>
#include <ios>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int part_1(vector<char> input) {
  int floor = 0;

  for (char ch : input) {
    if (ch == '(') {
      floor += 1;
    } else if (ch == ')') {
      floor -= 1;
    }
  }

  return floor;
}

int part_2(vector<char> input) {
  int floor = 0;

  for (int i = 0; i < input.size(); i++) {
    char ch = input[i];
    if (ch == '(') {
      floor += 1;
    } else if (ch == ')') {
      floor -= 1;
    }

    if (floor == -1) {
      return i + 1;
    }
  }
  return -1;
}

int main() {
  vector<char> input;
  fstream file("../input.txt", fstream::in);
  char ch;
  while (file >> noskipws >> ch) {
    input.push_back(ch);
  }
  file.close();

  cout << "Part 1: " << part_1(input) << endl;
  cout << "Part 2: " << part_2(input) << endl;

  return 0;
}
