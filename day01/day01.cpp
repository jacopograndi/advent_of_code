#include <iostream>
#include <string>
#include <vector> 
#include <algorithm> 

std::string read (char* name) {
	FILE *f = fopen(name, "r");
	char c = fgetc(f); std::string txt;
	while (c != EOF) { txt += c; c = fgetc(f); }
	fclose(f);
	return txt;
}

std::vector<std::string> split (std::vector<std::string> vec, std::string name) {
	auto token = name.find("\n");
	if (token != std::string::npos) {
		vec.push_back(name.substr(0, token));
		return split(vec, name.substr(token+1));
	} else if (name.size() > 1) { 
		vec.push_back(name);
	}
	return vec;
}

int main (int argc, char* argv[]) {
	if (argc < 2 && argc > 3) return 1;
	std::string txt = read(argv[1]);

	std::vector<std::string> vec;
	vec = split(vec, txt);
	std::vector<int> depths;
	for (auto v : vec) {
		depths.push_back(std::stoi(v));
	}

	int window = 1;
	if (argc == 3) { window = atoi(argv[2]); }

	int inc = 0;
	for (std::size_t i=0; i<depths.size(); i++) {
		if (i >= window) {
			int sum0 = 0, sum1 = 0;
			for (int j=0; j<window; j++) sum0 += depths[i-j-1];
			for (int j=0; j<window; j++) sum1 += depths[i-j];
			if (sum1 > sum0) inc ++;
		}
	}

	std::cout << "The number of depth increases is " << inc << std::endl;

	return 0;
}
