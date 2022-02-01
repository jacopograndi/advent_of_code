#include <iostream>
#include <fstream>
#include <vector>
#include <map>


long bin_to_int (std::string bin) {
	long num = 0, exp = 1;
	for (int i=bin.size()-1; i>=0; i--) { 
		int bit = bin[i] == '1' ? 1 : 0;
		num += bit * exp;
		exp *= 2;
	}
	return num;
}

std::string hex_to_bin (std::string hex) {
	std::map<char, std::string> convert {
		{ '0', "0000" }, { '1', "0001" }, { '2', "0010" }, { '3', "0011" },
		{ '4', "0100" }, { '5', "0101" }, { '6', "0110" }, { '7', "0111" },
		{ '8', "1000" }, { '9', "1001" }, { 'A', "1010" }, { 'B', "1011" },
		{ 'C', "1100" }, { 'D', "1101" }, { 'E', "1110" }, { 'F', "1111" },
	};
	std::string bin;
	for (char c : hex) { bin += convert[c]; }
	return bin;
}

long parse_literal (std::string raw) {
	std::string lit;
	for (int i=0; i<raw.size(); i++) {
		auto group = raw.substr(i*5, 5);
		lit += group.substr(1, 4);
		if (group[0] == '0') break;
	}
	return bin_to_int(lit);
}

class Packet {
	public: 
	Packet () { }

	int version;
	int id;
	long literal;
	std::vector<Packet> packets;

	int parse (std::string raw) {
		int parsed = 0;
		version = bin_to_int(raw.substr(0, 3));
		id = bin_to_int(raw.substr(3, 3));
		parsed += 6;


		if (id == 4) {
			std::string lit;
			for (int i=0; ; i++) {
				auto group = raw.substr(parsed, 5);
				parsed += 5;
				lit += group.substr(1, 4);
				if (group[0] == '0') break;
			}
			literal = bin_to_int(lit);
		} else {
			parsed += 1;
			if (raw[6] == '0') {
				int len = bin_to_int(raw.substr(7, 15));
				parsed += 15;
				int local = 0;
				while (local < len) {
					Packet pack;
					int size = pack.parse(raw.substr(parsed));
					local += size;
					parsed += size;
					packets.push_back(pack);
				}
			} else {
				int len = bin_to_int(raw.substr(7, 11));
				parsed += 11;
				int local = 0;
				while (local < len) {
					Packet pack;
					int size = pack.parse(raw.substr(parsed));
					local += 1;
					parsed += size;
					packets.push_back(pack);
				}
			}
		}
		return parsed;
	}

	int sum_version () {
		int s = version;
		for (Packet pack : packets) { s += pack.sum_version(); }
		return s;
	}

	long eval () {
		long res = 0;
		if (id == 4) { 
			res = literal;
	   	}
		else if (id == 5) {
			res = packets[0].eval() > packets[1].eval() ? 1 : 0;
		}
		else if (id == 6) {
			res = packets[0].eval() < packets[1].eval() ? 1 : 0;
		}
		else if (id == 7) {
			res = packets[0].eval() == packets[1].eval() ? 1 : 0;
		}
		else {
			bool first = true;
			for (Packet pack : packets) { 
				if (first) {
					res = pack.eval();
					first = false;
					continue;
				}
				if (id == 0) res += pack.eval();
				if (id == 1) res *= pack.eval();
				if (id == 2) res = std::min(res, pack.eval());
				if (id == 3) res = std::max(res, pack.eval());
			}
		}
		return res;
	}
};


int main (int argc, char *argv[]) {
	std::string raw;
	if (std::string({ argv[1] }) == "-i") {
		raw = std::string({ argv[2] });
	} else {
		std::getline(std::ifstream(argv[1]), raw, '\0');
	}
	std::cout << raw << std::endl;

	std::string bin = hex_to_bin(raw);

	std::cout << bin << std::endl;
	Packet packet;
	packet.parse(bin);

	std::cout << "version sum: "  <<  packet.sum_version() << std::endl;
	std::cout << "eval to: " << packet.eval() << std::endl;

	return 0;
}
