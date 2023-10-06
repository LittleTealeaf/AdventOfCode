#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

int find_marker(vector<char> &chars, int marker_size) {
  for (int i = 0; i < chars.size() - marker_size; i++) {
    // Check if there are any matches in the next 4 items
    bool match = true;
    for (int j = 0; j < marker_size && match; j++) {
      for (int k = j + 1; k < marker_size && match; k++) {
        if (chars[i + j] == chars[i + k]) {
          match = false;
        }
      }
    }

    if (match) {
      return i + marker_size;
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

  cout << "Part 1: " << find_marker(characters, 4) << endl;
  cout << "Part 2: " << find_marker(characters, 14) << endl;
}
