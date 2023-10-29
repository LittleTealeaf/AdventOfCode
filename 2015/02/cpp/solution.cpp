
#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int part_1(vector<string> input) {
  int total = 0;

  for (string line : input) {
    int i = line.find('x');
    int x = stoi(line.substr(0, i));
    string rem = line.substr(i + 1);
    i = rem.find('x');
    int y = stoi(rem.substr(0, i));
    rem = rem.substr(i + 1);
    int z = stoi(rem);

    int dims[] = {x, y, z};
    int n = sizeof(dims) / sizeof(dims[0]);

    sort(dims, dims + n);

    total += dims[0] * dims[1];

    total += 2 * x * y + 2 * x * z + 2 * y * z;
  }

  return total;
}

int part_2(vector<string> input) {
  int total = 0;

  for (string line : input) {
    int i = line.find('x');
    int x = stoi(line.substr(0, i));
    string rem = line.substr(i + 1);
    i = rem.find('x');
    int y = stoi(rem.substr(0, i));
    rem = rem.substr(i + 1);
    int z = stoi(rem);

    int dims[] = {x, y, z};
    int n = sizeof(dims) / sizeof(dims[0]);
    sort(dims, dims + n);

    total += dims[0] + dims[0] + dims[1] + dims[1];
    total += dims[0] * dims[1] * dims[2];
  }

  return total;
}

int main() {
  vector<string> input;
  ifstream file("../input.txt");
  string line;
  while (getline(file, line)) {
    input.push_back(line);
  }
  file.close();

  cout << "Part 1: " << part_1(input) << endl;
  cout << "Part 2: " << part_2(input) << endl;
}
