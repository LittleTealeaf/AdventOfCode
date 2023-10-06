#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

int part_1(vector<vector<int>> &grid) {

  int rows = grid.size(), cols = grid[0].size();

  // Visibilitiy Tracking
  bool **vis = new bool *[rows];

  for (int i = 0; i < rows; i++) {
    vis[i] = new bool[cols];
  }

  // Initialize, with borders being true
  for (int y = 0; y < rows; y++) {
    for (int x = 0; x < cols; x++) {
      vis[y][x] = y == 0 || y == rows - 1 || x == 0 || x == cols - 1;
    }
  }

  int height;

  for (int y = 1; y < rows - 1; y++) {
    // Start from the left
    height = grid[y][0];
    for (int x = 1; x < cols - 1; x++) {
      if (grid[y][x] > height) {
        vis[y][x] = true;
        height = grid[y][x];
        if (height == 9) {
          break;
        }
      }
    }

    // Start from the right
    height = grid[y][cols - 1];
    for (int x = cols - 2; x > 0; x--) {
      if (grid[y][x] > height) {
        vis[y][x] = true;
        height = grid[y][x];
        if (height == 9) {
          break;
        }
      }
    }
  }

  for (int x = 1; x < cols - 1; x++) {
    // Start from the left
    height = grid[0][x];
    for (int y = 1; y < rows - 1; y++) {
      if (grid[y][x] > height) {
        vis[y][x] = true;
        height = grid[y][x];
        if (height == 9) {
          break;
        }
      }
    }

    // Start from the right
    height = grid[rows - 1][x];
    for (int y = rows - 2; y > 0; y--) {
      if (grid[y][x] > height) {
        vis[y][x] = true;
        height = grid[y][x];
        if (height == 9) {
          break;
        }
      }
    }
  }

  int count = 0;

  for (int y = 0; y < rows; y++) {
    for (int x = 0; x < cols; x++) {
      if (vis[y][x]) {
        count += 1;
      }
    }
  }

  return count;
}

int part_2(vector<vector<int>> &grid) {
  int rows = grid.size(), cols = grid[0].size();

  int high_score = 0;

  for (int y = 0; y < rows; y++) {
    for (int x = 0; x < cols; x++) {
      int score = 0;
      // Going down
      for (int dy = 1; y + dy < rows; dy++) {
        score += 1;
        if (grid[y + dy][x] >= grid[y][x]) {
          break;
        }
      }

      int sub_score = 0;
      for (int dy = 1; y - dy >= 0; dy++) {
        sub_score += 1;
        if (grid[y - dy][x] >= grid[y][x]) {
          break;
        }
      }

      score *= sub_score;
      sub_score = 0;
      for (int dx = 1; x + dx < cols; dx++) {
        sub_score += 1;
        if (grid[y][x + dx] >= grid[y][x]) {
          break;
        }
      }

      score *= sub_score;
      sub_score = 0;
      for (int dx = 1; x - dx >= 0; dx++) {
        sub_score += 1;
        if (grid[y][x - dx] >= grid[y][x]) {
          break;
        }
      }
      score *= sub_score;

      if (score > high_score) {
        high_score = score;
      }
    }
  }

  return high_score;
}

int main() {
  vector<vector<int>> grid;

  ifstream file("../input.txt");
  if (!file.is_open()) {
    return 1;
  }

  string line;
  while (getline(file, line)) {
    vector<int> row;
    for (int i = 0; i < line.size(); i++) {
      row.push_back(line[i] - '0');
    }
    grid.push_back(row);
  }

  file.close();

  cout << "Part 1: " << part_1(grid) << endl;
  cout << "Part 2: " << part_2(grid) << endl;
}
