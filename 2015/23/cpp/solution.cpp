

#include <fstream>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int part_1(vector<string> input) {
  int a = 0;
  int b = 0;
  int pointer = 0;

  while (pointer < input.size()) {
    string line = input[pointer];
    if (line[0] == 'i') {
      if (line[4] == 'a') {
        a += 1;
      } else {
        b += 1;
      }
      pointer++;
    } else if (line[0] == 't') {
      if (line[4] == 'a') {
        a *= 3;
      } else {
        b *= 3;
      }
      pointer++;
    } else if (line[0] == 'h') {
      if (line[4] == 'a') {
        a /= 2;
      } else {
        b /= 2;
      }
      pointer++;
    } else if (line[1] == 'm') {
      string sub = line.substr(4);
      int offset = stoi(sub);
      pointer += offset;
    } else if (line[2] == 'e') {
      string sub = line.substr(7);
      int offset = stoi(sub);
      if ((line[4] == 'a' && a % 2 == 0) || (line[4] == 'b' && b % 2 == 0)) {
        pointer += offset;
      } else {
        pointer++;
      }
    } else {
      string sub = line.substr(7);
      int offset = stoi(sub);
      if ((line[4] == 'a' && a == 1) || (line[4] == 'b' && b == 1)) {
        pointer += offset;
      } else {
        pointer++;
      }
    }
  }

  return b;
}

int part_2(vector<string> input) {
  int a = 1;
  int b = 0;
  int pointer = 0;

  while (pointer < input.size()) {
    string line = input[pointer];
    if (line[0] == 'i') {
      if (line[4] == 'a') {
        a += 1;
      } else {
        b += 1;
      }
      pointer++;
    } else if (line[0] == 't') {
      if (line[4] == 'a') {
        a *= 3;
      } else {
        b *= 3;
      }
      pointer++;
    } else if (line[0] == 'h') {
      if (line[4] == 'a') {
        a /= 2;
      } else {
        b /= 2;
      }
      pointer++;
    } else if (line[1] == 'm') {
      string sub = line.substr(4);
      int offset = stoi(sub);
      pointer += offset;
    } else if (line[2] == 'e') {
      string sub = line.substr(7);
      int offset = stoi(sub);
      if ((line[4] == 'a' && a % 2 == 0) || (line[4] == 'b' && b % 2 == 0)) {
        pointer += offset;
      } else {
        pointer++;
      }
    } else {
      string sub = line.substr(7);
      int offset = stoi(sub);
      if ((line[4] == 'a' && a == 1) || (line[4] == 'b' && b == 1)) {
        pointer += offset;
      } else {
        pointer++;
      }
    }
  }

  return b;
}

int main() {
  vector<string> lines;
  ifstream file("../input.txt");

  string line;
  while (getline(file, line)) {
    lines.push_back(line);
  }

  cout << "Part 1: " << part_1(lines) << endl;
  cout << "Part 2: " << part_2(lines) << endl;

  return 0;
}
