
#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int part_1(vector<string> input) {
  int sum = 0;

  for (string line : input) {
    int ind_start = 5;
    int ind_end = line.find(':');

    int id = stoi(line.substr(ind_start, ind_end - ind_start));
    string segment = line.substr(ind_end + 2);

    bool possible = true;

    while (segment.length() > 0) {
      int next = min(segment.find(','), segment.find(';'));
      string token = segment.substr(0, next);
      if (next == -1) {
        token = segment.substr(0);
        segment = "";
      }

      int index = token.find(' ');
      int count = stoi(token.substr(0, index));
      string color = token.substr(index + 1);

      bool valid = (color != "red" || count <= 12) &&
                   (color != "green" || count <= 13) &&
                   (color != "blue" || count <= 14);
      if (!valid) {
        possible = false;
        break;
      }

      if (next != -1) {
        segment = segment.substr(next + 2);
      }
    }

    if (possible) {
      sum += id;
    }
  }

  return sum;
}

int part_2(vector<string> input) {
  int sum = 0;

  for (string line : input) {
    int ind_start = 5;
    int ind_end = line.find(':');

    int id = stoi(line.substr(ind_start, ind_end - ind_start));
    string segment = line.substr(ind_end + 2);

    int red = 0;
    int green = 0;
    int blue = 0;

    while (segment.length() > 0) {
      int next = min(segment.find(','), segment.find(';'));
      string token = segment.substr(0, next);
      if (next == -1) {
        token = segment.substr(0);
        segment = "";
      }

      int index = token.find(' ');
      int count = stoi(token.substr(0, index));
      string color = token.substr(index + 1);

      if (color == "red") {
        red = max(red, count);
      } else if (color == "green") {
        green = max(green, count);
      } else if (color == "blue") {
        blue = max(blue, count);
      }

      if (next != -1) {
        segment = segment.substr(next + 2);
      }
    }

    sum += red * green * blue;
  }

  return sum;
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
