#include <iostream>
#include <fstream>
#include <vector>
#include <stack>
#include <string>
#include <algorithm>

#include "../utils.h"

char map (char a) {
	if (a == '(') return ')';
	if (a == '[') return ']';
	if (a == '{') return '}';
	if (a == '<') return '>';
	return 'A';
}

bool match (char a, char b) {
	if (a == '(' && b == ')') return true;
	if (a == '[' && b == ']') return true;
	if (a == '{' && b == '}') return true;
	if (a == '<' && b == '>') return true;
	return false;
}

int score (char a) {
	if (a == ')') return 3;
	if (a == ']') return 57;
	if (a == '}') return 1197;
	if (a == '>') return 25137;
	return 0;
}

long score (std::string str) {
	long s = 0;
	for (char c : str) {
		s *= 5;
		s += (c == ')' ? 1 : 0);
		s += (c == ']' ? 2 : 0);
		s += (c == '}' ? 3 : 0);
		s += (c == '>' ? 4 : 0);
	}
	return s;
}

bool check (std::string line, std::string &out) {
	std::stack<char> stack;
	for (char c : line) {
		if (c == '(' || c == '[' || c == '{' || c == '<') {
			stack.push(c);
		} else {
			if (!match(stack.top(), c)) {
				out.push_back(c);
				return true;
			} else stack.pop();
		}
	}
	while (!stack.empty()) {
		out.push_back(map(stack.top()));
		stack.pop();
	}
	return false;
}

int main (int argc, char *argv[]) {
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	std::vector<std::string> lines;
	split(lines, raw, "\n");

	std::vector<long> repairs;
	int sum = 0;
	for (auto line : lines) {
		if (line.size() == 0) continue;
		std::string rep;
		bool corrupt = check(line, rep);
		if (corrupt) sum += score(rep[0]);
		else {
			repairs.push_back(score(rep));
			std::cout << score(rep) << " " << rep << std::endl;
		}
	}
	std::cout << "corrupt score: " << sum << std::endl;
	std::sort(std::begin(repairs), std::end(repairs));
	std::cout << "repairs score: " << repairs[(repairs.size()-1)/2] << std::endl;

	return 0;
}
