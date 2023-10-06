#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

int part_1(vector<char> &chars) {
  for (int i = 0; i < chars.size() - 4; i++) {
    // Check if there are any matches in the next 4 items
    bool match = true;
    for (int j = 0; j < 4 && match; j++) {
      for (int k = j + 1; k < 4 && match; k++) {
        if (chars[i + j] == chars[i + k]) {
          match = false;
        }
      }
    }

    if (match) {
      return i + 4;
    }
  }
  return -1;
}

int part_2(vector<char> &chars) {
  for (int i = 0; i < chars.size() - 14; i++) {
    // Check if there are any matches in the next 4 items
    bool match = true;
    for (int j = 0; j < 14 && match; j++) {
      for (int k = j + 1; k < 14 && match; k++) {
        if (chars[i + j] == chars[i + k]) {
          match = false;
        }
      }
    }

    if (match) {
      return i + 14;
    }
  }
  return -1;
}

int main() {
  vector<char> characters;

  ifstream file("../input.txt");
  if (!file.is_open()) {
    return 1;
  }

  char c;
  while (file.get(c)) {
    characters.push_back(c);
  }

  file.close();

  cout << "Part 1: " << part_1(characters) << endl;
  cout << "Part 2: " << part_2(characters) << endl;
}
