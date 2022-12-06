#include <stdio.h>

int leaderboard[3];
int current;

int line;
int code;

int main(int argc, char *argv[]) {
	
	do {
		code = scanf("%d",&line);

		printf("%d",code);

		if(code == 1) {
			if(leaderboard[0] < current) {
				leaderboard[0] = current;
			}	
		} else {
			current += line;
		}

	} while(code != EOF);

	printf("%d",leaderboard[0]);

	return 0;
}
