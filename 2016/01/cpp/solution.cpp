#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
using namespace std;

int part_1(string input) {
  string str = input;
  int x = 0;
  int y = 0;
  int direction = 3;
  bool last = false;

  while (str.find(',') != -1 || (last = true)) {
    char dir = str[0];

    if (dir == 'L') {
      direction = (direction + 3) % 4;
    } else if (dir == 'R') {
      direction = (direction + 1) % 4;
    }

    string sub_num = str.substr(1, str.find(','));
    int steps = stoi(sub_num);

    if (direction == 0) {
      x += steps;
    } else if (direction == 1) {
      y -= steps;
    } else if (direction == 2) {
      x -= steps;
    } else if (direction == 3) {
      y += steps;
    }

    if (last) {
      break;
    } else {
      str = str.substr(str.find(" ") + 1);
    }
  }

  if (x < 0) {
    x *= -1;
  }

  if (y < 0) {
    y *= -1;
  }

  return x + y;
}

struct Position {
  int x;
  int y;
};

bool pos_equal(Position a, Position b) { return a.x == b.x && a.y == b.y; }

bool contains(vector<Position> vec, Position pos) {
  for (Position p : vec) {
    if (pos_equal(p, pos)) {
      return true;
    }
  }
  return false;
}

int part_2(string input) {
  string str = input;
  int x = 0;
  int y = 0;
  int direction = 3;
  bool last = false;

  vector<Position> positions;

  while (str.find(',') != -1 || (last = true)) {
    char dir = str[0];

    if (dir == 'L') {
      direction = (direction + 3) % 4;
    } else if (dir == 'R') {
      direction = (direction + 1) % 4;
    }

    string sub_num = str.substr(1, str.find(','));
    int steps = stoi(sub_num);

    for (int i = 0; i < steps; i++) {
      if (direction == 0) {
        x += 1;
      } else if (direction == 1) {
        y -= 1;
      } else if (direction == 2) {
        x -= 1;
      } else if (direction == 3) {
        y += 1;
      }

      Position pos = Position{.x = x, .y = y};

      if (contains(positions, pos)) {
        if (x < 0) {
          x *= -1;
        }

        if (y < 0) {
          y *= -1;
        }
        return x + y;
      } else {
        positions.push_back(pos);
      }
    }

    if (last) {
      break;
    } else {
      str = str.substr(str.find(" ") + 1);
    }
  }

  return -1;
}

int main() {
  ifstream file("../input.txt");
  stringstream buffer;
  buffer << file.rdbuf();
  string input = buffer.str();

  cout << "Part 1: " << part_1(input) << endl;
  cout << "Part 2: " << part_2(input) << endl;

  return 0;
}
