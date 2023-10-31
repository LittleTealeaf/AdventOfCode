
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int calculate(vector<int> input, int iterations) {
  vector<int> val = input;

  for (int i = 0; i < iterations; i++) {
    vector<int> n_val;

    int current = val[0];
    int count = 1;

    for (int j = 1; j < val.size(); j++) {
      if (val[j] == current) {
        count++;
      } else {
        n_val.push_back(count);
        n_val.push_back(current);
        current = val[j];
        count = 1;
      }
    }
    n_val.push_back(count);
    n_val.push_back(current);

    val = n_val;
  }

  return val.size();
}

int main() {

  vector<int> input = {1, 3, 2, 1, 1, 3, 1, 1, 1, 2};

  cout << "Part 1: " << calculate(input, 40) << endl;
  cout << "Part 2: " << calculate(input, 50) << endl;

  return 0;
}
