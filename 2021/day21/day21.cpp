#include <iostream>
#include <string>

long long winp = 0, winq = 0;

void stepseq (int p, int q, int sp, int sq, int turn, int roll, int die) {
	if (turn == 0) {
		if (roll < 3) {
			stepseq((p+die)%10, q, sp, sq, turn, roll+1, die+1);
		} else {
			int score = p==0 ? sp+10 : sp+p;
			if (score >= 1000) std::cout << "win sq*die: " << sq*(die-1) << std::endl;
			else stepseq(p, q, score, sq, (turn+1)%2, 0, die);
		}
	}
	if (turn == 1) {
		if (roll < 3) {
			stepseq(p, (q+die)%10, sp, sq, turn, roll+1, die+1);
		} else {
			int score = q==0 ? sq+10 : sq+q;
			if (score >= 1000) std::cout << "win sp*die: " << sp*(die-1) << std::endl;
			else stepseq(p, q, sp, score, (turn+1)%2, 0, die);
		}
	}
}

void step (int p, int q, int sp, int sq, int turn, long mul) {
	//std::cout << p<<" "<<q<< " "<<sp << " " <<sq << ",,,, "<<turn<<" "<<mul<<std::endl;
	int incp = p==0 ? 10 : p;
	int incq = q==0 ? 10 : q;
	if (sp+incp >= 21) { winp += mul; return; }
	if (sq+incq >= 21) { winq += mul; return; }
	if (turn % 2 == 0) {
		int score = sp+incp;
		if (turn == 0) score = 0;
		step((p+3)%10, q, score, sq, turn+1, mul*1);
		step((p+4)%10, q, score, sq, turn+1, mul*3);
		step((p+5)%10, q, score, sq, turn+1, mul*6);
		step((p+6)%10, q, score, sq, turn+1, mul*7);
		step((p+7)%10, q, score, sq, turn+1, mul*6);
		step((p+8)%10, q, score, sq, turn+1, mul*3);
		step((p+9)%10, q, score, sq, turn+1, mul*1);
	}
	if (turn % 2 == 1) {
		int score = sq+incq;
		if (turn == 1) score = 0;
		step(p, (q+3)%10, sp, score, turn+1, mul*1);
		step(p, (q+4)%10, sp, score, turn+1, mul*3);
		step(p, (q+5)%10, sp, score, turn+1, mul*6);
		step(p, (q+6)%10, sp, score, turn+1, mul*7);
		step(p, (q+7)%10, sp, score, turn+1, mul*6);
		step(p, (q+8)%10, sp, score, turn+1, mul*3);
		step(p, (q+9)%10, sp, score, turn+1, mul*1);
	}
}

int main (int argc, char *argv[]) {
	int p = std::stoi(std::string{ argv[1] });
	int q = std::stoi(std::string{ argv[2] });
	stepseq(p, q, 0, 0, 0, 0, 1);
	step(p, q, 0, 0, 0, 1);
	std::cout << "wins " << winp << " " << winq << std::endl;

	return 0;
}
