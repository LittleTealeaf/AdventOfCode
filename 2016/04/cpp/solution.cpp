#include <fstream>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int part_1(vector<string> lines) {
  int sum = 0;

  for (string line : lines) {

    int counts[26] = {};
    int start_id = -1;
    int start_checksum = -1;

    for (int i = 0; i < line.length(); i++) {
      char c = line[i];
      if (c >= 'a' && c <= 'z') {
        counts[c - 'a'] += 1;
      } else if (start_id == -1 && c >= '0' && c <= '9') {
        start_id = i;
      } else if (c == '[') {
        start_checksum = i;
        break;
      }
    }

    int leaderboard[5] = {-1, -1, -1, -1, -1};

    for (int i = 0; i < 26; i++) {
      for (int j = 0; j < 5; j++) {
        if (leaderboard[j] == -1) {
          leaderboard[j] = i;
          break;
        } else if (counts[leaderboard[j]] < counts[i]) {
          for (int k = 4; k >= j + 1; k--) {
            leaderboard[k] = leaderboard[k - 1];
          }
          leaderboard[j] = i;
          break;
        }
      }
    }

    bool verified = true;

    for (int i = 0; i < 5; i++) {
      if (leaderboard[i] + 'a' != line[start_checksum + i + 1]) {
        verified = false;
      }
    }

    if (verified) {
      sum += stoi(line.substr(start_id, start_checksum));
    }
  }

  return sum;
}

int part_2(vector<string> lines) {
  for (string line : lines) {

    int counts[26] = {};
    int start_id = -1;
    int start_checksum = -1;

    for (int i = 0; i < line.length(); i++) {
      char c = line[i];
      if (c >= 'a' && c <= 'z') {
        counts[c - 'a'] += 1;
      } else if (start_id == -1 && c >= '0' && c <= '9') {
        start_id = i;
      } else if (c == '[') {
        start_checksum = i;
        break;
      }
    }

    int leaderboard[5] = {-1, -1, -1, -1, -1};

    for (int i = 0; i < 26; i++) {
      for (int j = 0; j < 5; j++) {
        if (leaderboard[j] == -1) {
          leaderboard[j] = i;
          break;
        } else if (counts[leaderboard[j]] < counts[i]) {
          for (int k = 4; k >= j + 1; k--) {
            leaderboard[k] = leaderboard[k - 1];
          }
          leaderboard[j] = i;
          break;
        }
      }
    }

    bool verified = true;

    for (int i = 0; i < 5; i++) {
      if (leaderboard[i] + 'a' != line[start_checksum + i + 1]) {
        verified = false;
      }
    }

    if (verified) {
      int id = stoi(line.substr(start_id, start_checksum));

      string str = "";

      for (int i = 0; i < start_id - 1; i++) {
        if (line[i] == '-') {
          str.push_back('-');
        } else {
          char c = (line[i] - 'a' + id) % 26 + 'a';
          str.push_back(c);
        }
      }

      if (str == "northpole-object-storage") {
        return id;
      }
    }
  }

  return -1;
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
