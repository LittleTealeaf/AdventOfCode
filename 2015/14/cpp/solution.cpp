

#include <fstream>
#include <iostream>
#include <iterator>
#include <string>
#include <vector>
using namespace std;

struct Reindeer {
  int speed;
  int duration;
  int rest;
};

vector<Reindeer> parseInput(vector<string> input) {
  vector<Reindeer> values;
  for (string line : input) {
    int i = line.find(' ');
    string str = line.substr(i + 9);
    i = str.find(' ');
    int speed = stoi(str.substr(0, i));
    str = str.substr(i + 10);
    i = str.find(' ');
    int duration = stoi(str.substr(0, i));
    str = str.substr(i + 33);
    i = str.find(' ');
    int rest = stoi(str.substr(0, i));

    values.push_back(Reindeer{speed, duration, rest});
  }
  return values;
}

int part_1(vector<Reindeer> reindeer, int seconds) {
  int max = 0;

  for (Reindeer r : reindeer) {
    int cycles = seconds / (r.duration + r.rest);
    int seconds_remaining = seconds - (r.duration + r.rest) * cycles;

    int duration = 0;

    if (seconds_remaining >= r.duration) {
      cycles += 1;
    } else {
      duration += seconds_remaining * r.speed;
    }

    duration += cycles * r.duration * r.speed;

    if (duration > max) {
      max = duration;
    }
  }

  return max;
}

int part_2(vector<Reindeer> reindeer, int seconds) {
  vector<int> points;
  vector<int> distances;
  for (int i = 0; i < reindeer.size(); i++) {
    points.push_back(0);
    distances.push_back(0);
  }

  for (int i = 0; i < seconds; i++) {
    for (int j = 0; j < reindeer.size(); j++) {
      Reindeer r = reindeer[j];
      int cycle = i % (r.duration + r.rest);
      if (cycle < r.duration) {
        distances[j] += r.speed;
      }
    }
    int max = 0;
    int max_i = 0;

    for (int j = 0; j < reindeer.size(); j++) {
      if (distances[j] > max) {
        max = distances[j];
        max_i = j;
      }
    }

    points[max_i] += 1;
  }

  int max = 0;

  for (int p : points) {
    if (max < p) {
      max = p;
    }
  }

  return max;
}

int main() {
  vector<string> lines;
  ifstream file("../input.txt");
  string line;
  while (getline(file, line)) {
    lines.push_back(line);
  }

  vector<Reindeer> reindeer = parseInput(lines);

  cout << "Part 1: " << part_1(reindeer, 2503) << endl;
  cout << "Part 2: " << part_2(reindeer, 2503) << endl;

  return 0;
}
