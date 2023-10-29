
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int part_1(vector<string> input) {
  bool state[1000][1000];
  for (int x = 0; x < 1000; x++) {
    for (int y = 0; y < 1000; y++) {
      state[x][y] = false;
    }
  }

  for (string line : input) {
    bool set = false, toggle = false;
    int start;
    if (line[1] == 'o') {
      toggle = true;
      start = 7;
    } else {
      set = line[6] == 'n';
      start = set ? 8 : 9;
    }
    int end = line.find(",");
    int s_x = stoi(line.substr(start, end));
    string str = line.substr(end + 1);
    end = str.find(' ');
    int s_y = stoi(str.substr(0, end));
    str = str.substr(end + 9);
    end = str.find(",");
    int e_x = stoi(str.substr(0, end));
    int e_y = stoi(str.substr(end + 1));

    for (int x = s_x; x <= e_x; x++) {
      for (int y = s_y; y <= e_y; y++) {
        state[x][y] = toggle ? !state[x][y] : set;
      }
    }
  }

  int count = 0;

  for (int x = 0; x < 1000; x++) {
    for (int y = 0; y < 1000; y++) {
      if (state[x][y]) {
        count += 1;
      }
    }
  }

  return count;
}

int part_2(vector<string> input) {
  int state[1000][1000];
  for (int x = 0; x < 1000; x++) {
    for (int y = 0; y < 1000; y++) {
      state[x][y] = 0;
    }
  }

  for (string line : input) {
    bool set = false, toggle = false;
    int start;
    if (line[1] == 'o') {
      toggle = true;
      start = 7;
    } else {
      set = line[6] == 'n';
      start = set ? 8 : 9;
    }
    int end = line.find(",");
    int s_x = stoi(line.substr(start, end));
    string str = line.substr(end + 1);
    end = str.find(' ');
    int s_y = stoi(str.substr(0, end));
    str = str.substr(end + 9);
    end = str.find(",");
    int e_x = stoi(str.substr(0, end));
    int e_y = stoi(str.substr(end + 1));

    for (int x = s_x; x <= e_x; x++) {
      for (int y = s_y; y <= e_y; y++) {
        if (toggle) {
          state[x][y] += 2;
        } else if (set) {
          state[x][y] += 1;
        } else if (state[x][y] > 0) {
          state[x][y] -= 1;
        }
      }
    }
  }

  int count = 0;

  for (int x = 0; x < 1000; x++) {
    for (int y = 0; y < 1000; y++) {
      count += state[x][y];
    }
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
