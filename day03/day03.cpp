#include <iostream>
#include <fstream>
#include <string>
#include <vector>

int bin_to_dec (std::string bin) {
	int dec = 0, pow = 1;
	for (std::size_t i=bin.size(); i > 0; i--) {
		dec += pow * (bin[i-1] == '0' ? 0 : 1);
		pow *= 2;
	}
	return dec;
}

std::vector<int> get_freq (std::vector<std::string> bits) { 
	std::vector<int> freq;
	for (std::size_t i=0; i<bits[0].size(); i++) {
		freq.push_back(0);
	}

	for (auto b : bits) {
		for (std::size_t i=0; i< b.size(); i++) {
		 	freq[i] += (b[i] == '0' ? 0 : 1)*2-1;
		}
	}
	return freq;
}

std::vector<std::string> filter (std::vector<std::string> bits, int inv, int sel) {
	std::vector<std::string> res;
	auto freq = get_freq(bits);

	for (auto b : bits) {
		int f = freq[sel] * inv;
		if (f == 0) f += inv;
		if ((f > 0 ? '0' : '1') == b[sel]) {
			res.push_back(b);
		}
	}

	return res;
}

int main (int argc, char *argv[]) {
	if (argc != 2) return 1;

	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');

	std::vector<std::string> bits;	

	while (1) {
		auto token = raw.find("\n");
		if (token != std::string::npos) {
			bits.push_back(raw.substr(0, token));
			raw = raw.substr(token+1);
		} else {
			if (raw.size() > 0) bits.push_back(raw);
			break;
		}
	}

	auto freq = get_freq(bits);

	std::string gamma = "", epsilon = "";
	for (std::size_t i=0; i<freq.size(); i++) {
		gamma += freq[i] > 0 ? "1" : "0";
		epsilon += freq[i] > 0 ? "0" : "1";
	}

	int g = bin_to_dec(gamma);
	int e = bin_to_dec(epsilon);
	std::cout << "power level " << e*g << ", "
   		<< "epsilon " << e << " (" << epsilon << "), "
		<< "gamma " << g << " (" << gamma << ")" << std::endl;

	std::vector<std::string> o2 = bits;
	for (std::size_t i=0; i<freq.size(); i++) {
		o2 = filter(o2, 1, i);
		if (o2.size() == 1) break;
	}

	std::vector<std::string> co2 = bits;
	for (std::size_t i=0; i<freq.size(); i++) {
		co2 = filter(co2, -1, i);
		if (co2.size() == 1) break;
	}

	int generator = bin_to_dec(o2[0]);
	int scrubber = bin_to_dec(co2[0]);
	std::cout << "oxigen generator rating " << scrubber*generator << ", "
   		<< "epsilon " << generator << " (" << o2[0] << "), "
		<< "gamma " << scrubber << " (" << co2[0] << ")" << std::endl;

	return 0;
}
