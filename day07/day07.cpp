#include <iostream>
#include <fstream>
#include <string>
#include <vector>

#include "../utils.h"

long sumdist (std::vector<int> pos, int pole, int inc) {
	long sd = 0;
	for (auto p : pos) { 
		int n = abs(p-pole); 
		if (inc == 1) n = (n * (n+1)) / 2;
		sd += n;
   	}
	return sd;
}

int main (int argc, char *argv[]) {
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	std::vector<std::string> vec;
	split(vec, raw, ",");
	std::vector<int> pos;
	for (auto s : vec) pos.push_back(std::stoi(s));
	
	int minp = 10000, maxp = -10000;
	for (auto p : pos) {
		minp = std::min(minp, p);
		maxp = std::max(maxp, p);
	}
	int inc = argc > 2 ? atoi(argv[2]) : 0;
	long fuel = 100000000000;
	for (int i=minp; i<maxp; i++) {
		fuel = std::min(fuel, sumdist(pos, i, inc));
	}
	
	std::cout << "fuel: " << fuel << std::endl;

	return 0;
}
