#include <fstream>
#include <iostream>
#include <string>
#include <valarray>
#include <vector>
using namespace std;

string part_1(vector<string> input) {
  string solution = "";

  for (string s : input) {
    int position = 4;

    for (char c : s) {
      if (c == 'U') {
        if (position > 2) {
          position -= 3;
        }
      } else if (c == 'D') {
        if (position < 6) {
          position += 3;
        }
      } else if (c == 'R') {
        if (position % 3 < 2) {
          position += 1;
        }
      } else if (c == 'L') {
        if (position % 3 > 0) {
          position -= 1;
        }
      }
    }

    solution.push_back('1' + position);
  }

  return solution;
}

string part_2(vector<string> input) {
  string solution = "";

  for (string s : input) {
    int x = 0;
    int y = 0;

    for (char c : s) {
      if (c == 'D') {
        if (y > abs(x) - 2) {
          y -= 1;
        }
      } else if (c == 'U') {
        if (y < -abs(x) + 2) {
          y += 1;
        }
      } else if (c == 'L') {
        if (x > abs(y) - 2) {
          x -= 1;
        }
      } else if (c == 'R') {
        if (x < -abs(y) + 2) {
          x += 1;
        }
      }
    }

    if (y == 2) {
      solution.push_back('1');
    } else if (y == 1) {
      solution.push_back(x + '3');
    } else if (y == 0) {
      solution.push_back(x + '7');
    } else if (y == -1) {
      solution.push_back(x + 'B');
    } else {
      solution.push_back('D');
    }
  }

  return solution;
}

int main() {
  vector<string> lines;
  string line;
  ifstream file("../input.txt");

  while (getline(file, line)) {
    lines.push_back(line);
  }

  file.close();

  cout << "Part 1: " << part_1(lines) << endl;
  cout << "Part 2: " << part_2(lines) << endl;

  return 0;
}
