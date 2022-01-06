#include <iostream>
#include <fstream>
#include <vector>
#include <string>

#include "../utils.h"

int distance (std::string a, std::string b) {
	int d = 0;
	for (auto k : b) {
		if (a.find(k) == std::string::npos) d++;
	} return d;
}

int map (std::vector<std::string> tries, std::string dig) {
	if (dig.size() == 2) return 1;
	if (dig.size() == 3) return 7;
	if (dig.size() == 4) return 4;
	if (dig.size() == 7) return 8;
	if (dig.size() == 4) return 8;
	
	std::string one = *std::find_if(std::begin(tries), std::end(tries), 
		[](std::string &str){ return str.size() == 2; });
	std::string four = *std::find_if(std::begin(tries), std::end(tries), 
		[](std::string &str){ return str.size() == 4; });
	if (dig.size() == 5) {
		if (distance(dig, one) == 0) return 3;
		if (distance(dig, four) == 1) return 5;
		return 2;
	}
	if (dig.size() == 6) {
		if (distance(dig, one) == 1) return 6;
		if (distance(dig, four) == 0) return 9;
	}
	return 0;
}

int main (int argc, char *argv[]) {
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	std::vector<std::string> lines, parts;
	split(lines, raw, "\n");

	std::vector<int> count;
	for (int i=0; i<8; i++) count.push_back(0);

	int sum = 0;
	for (auto line : lines) {
		if (line.size() == 0) continue;
		parts.clear();
		split(parts, line, " | ");	

		std::vector<std::string> tries, digits;
		split(tries, parts[0], " ");
		split(digits, parts[1], " ");

		int i = 1000;
		int num = 0;	
		for (auto dig : digits) {
			count[dig.size()]++;
			num += i*map(tries, dig);
			i/=10;
		}
		sum += num;
	}

	std::cout << "digits: " << count[2] + count[3] + count[4] + count[7] 
		<< std::endl;
	std::cout << "sum: " << sum << std::endl;

	return 0;
}
