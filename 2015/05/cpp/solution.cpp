#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int part_1(vector<string> lines) {
  int count = 0;

  for (string line : lines) {
    int vowels = 0;
    bool has_double = false;

    for (int i = 0; i < line.length(); i++) {
      if (string("aeiou").find(line[i]) != -1) {
        vowels += 1;
      }
      if (i > 0) {
        if (line[i - 1] == line[i]) {
          has_double = true;
        }
      }
    }

    bool has_illegal = false;
    string illegals[] = {string("ab"), string("cd"), string("pq"),
                         string("xy")};
    for (int i = 0; i < 4 && !has_illegal; i++) {
      string illegal = illegals[i];
      if (line.find(illegal) != -1) {
        has_illegal = true;
      }
    }

    if (has_illegal || vowels < 3 || !has_double) {
      continue;
    }

    count++;
  }

  return count;
}

int part_2(vector<string> input) {
  int count = 0;

  for (string line : input) {
    bool sandwich = false;
    bool repeat = false;
    for (int i = 0; i < line.length() && !(sandwich && repeat); i++) {
      if (i > 0) {
        for (int j = i + 2; j < line.length(); j++) {
          if (line[i] == line[j] && line[i - 1] == line[j - 1]) {
            repeat = true;
          }
        }
      }
      if (i > 1) {
        if (line[i - 2] == line[i]) {
          sandwich = true;
        }
      }
    }

    if (sandwich && repeat) {
      count += 1;
    }
  }

  return count;
}

int main() {
  vector<string> lines;
  fstream file("../input.txt");
  string line;
  while (getline(file, line)) {
    lines.push_back(line);
  }

  cout << "Part 1: " << part_1(lines) << endl;
  cout << "Part 2: " << part_2(lines) << endl;

  return 0;
}
