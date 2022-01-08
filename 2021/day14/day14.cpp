#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <map>
#include "../utils.h"

std::map<std::string, long> apply (std::map<std::string, long> pairs, 
	std::map<std::string, char> rules, std::map<char, long> &count) 
{
	std::map<std::string, long> next = pairs;
	for (auto &pair : pairs) {
		char c = rules[pair.first];
		count[c] += pair.second;
		std::string l; 
		l.push_back(pair.first[0]);
		l.push_back(c);
		std::string r; 
		r.push_back(c);
		r.push_back(pair.first[1]);
		next[pair.first] -= pair.second;
		next[l] += pair.second;
		next[r] += pair.second;
	}
	return next;
}

void to_pairs (std::string init, std::map<std::string, long> &pairs) {
	for (std::size_t i=0; i<init.size()-1; i++) {
		std::string seg = init.substr(i, 2);
		pairs[seg] ++;
	}
}

long tally (std::map<char, long> map) {
	std::vector<long> f;
	for (auto p : map) {
	       	f.push_back(p.second);
		std::cout << p.first << " " << p.second << std::endl;	
	}
	std::sort(f.begin(), f.end());
	return f[f.size()-1] - f[0];
}

int main (int argc, char *argv[]) {
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	std::vector<std::string> parts, strrules;
	split(parts, raw, "\n\n");
	split(strrules, parts[1], "\n");

	std::string polymer = parts[0];
	std::cout << "starting polymer: " << polymer << std::endl;

	std::map<std::string, long> pairs;
	std::map<std::string, char> rules;
	for (auto strrule : strrules) {
		if (strrule.size() == 0) continue;
		std::vector<std::string> rl;
		split(rl, strrule, " -> ");
		rules[rl[0]] = rl[1][0];
		pairs[rl[0]] = 0;
	}

	to_pairs(polymer, pairs);
	
	std::map<char, long> count;
	for (auto pair : pairs) {
		count[pair.first[0]] = 0;
		count[pair.first[1]] = 0;
	}

	for (char c : polymer) {
		count[c] ++;
	}

	long iter = 10;
	if (argc > 2) iter = atoi(argv[2]);
	for (long i=0; i<iter; i++) {
		pairs = apply(pairs, rules, count);
	}

	long res = tally(count);
	std::cout << "most common - least common: " << res << std::endl;

	return 0;
}
