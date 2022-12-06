#include <stdio.h>
#include <string.h>

char line[10];
int max_calories, current_calories;

int main(int argc, char *argv[]) {

	current_calories = 0;

  while (scanf("%10s", line) != EOF) {
    if (strlen(line) > 0) {
			printf("%s\t%lu\n",line, strlen(line));
      int x;
      sscanf(line, "%d", &x);
      current_calories += x;
    } else {
			printf("Elf with %d calories", current_calories);
      if (current_calories > max_calories) {
        max_calories = current_calories;
      }
			current_calories = 0;
    }
  }

  printf("%d\n", max_calories);

  return 0;
}
