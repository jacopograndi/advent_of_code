#include <iostream>
#include <fstream>
#include <string>
#include <vector>

#include "../utils.h"

long sum (std::vector<long> vec) {
	long s = 0; for (long v : vec) s += v; return s;
}

int main (int argc, char *argv[]) {
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	std::vector<std::string> strfish;
	split(strfish, raw, ",");
	
	std::vector<long> fish;
	for (int i=0; i<9; i++) { fish.push_back(0); }
	for (std::string str : strfish) fish[std::stoi(str)] += 1;

	int days = 80;
	if (argc > 2) days = std::stoi(std::string(argv[2]));
	for (int i=0; i<days; i++) {
		long born = fish[0];
		fish[0] = 0;
		for (int j=1; j<9; j++) {
			fish[j-1] += fish[j];
			fish[j] = 0;
		}
		fish[6] += born;
		fish[8] = born;
	
		if (argc > 3 && std::string(argv[3]) == "-w") {	
			std::cout << "day " << i+1 << "/" << days 
				<< " current fish: " << sum(fish) << std::endl;
		}
		if (argc > 3 && std::string(argv[3]) == "-v") {	
			std::cout << "day " << i+1 << ": ";
			for (auto f : fish) std::cout << f << ",";
			std::cout << std::endl;
		}
	}

	std::cout << "fish: " << sum(fish) << std::endl;
	return 0;
}
