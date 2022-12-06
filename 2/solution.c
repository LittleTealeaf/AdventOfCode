#include <stdio.h>


int opponent, you;
char val;
int score;

int main(int argc, char *argv[]) {
	
	score = 0;
	while(scanf("%s",&val) != EOF) {
		if(opponent == -1) {
			opponent = ((int) val) - 'A';
		} else {
			you = ((int) val) - 'X';

			score += you + 1;
			
			if(you - opponent == 1 || (you == 0 && opponent == 2)) {
				score += 6;
			} else if(you == opponent) {
				score += 3;
			}

			opponent = -1;
		}
	}
	printf("%d\n",score);
}

/*
char opponent, current;

int score;

int main(int argc, char *argv[]) {
	score = 0;
	while(scanf("%s",&current) != EOF) {
		if(opponent == '\0') {
			opponent = current;
		} else {
			score += ((int) current) - 'X' + 1;
			
			int a = ((int) opponent) - 'A';
			int b = ((int) current) - 'X';



			opponent = '\0';
		}
	}
}
*/
