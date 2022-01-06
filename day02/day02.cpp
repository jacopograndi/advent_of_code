#include <iostream>

int main (int argc, char *argv[]) {
	if (argc != 2) return 1;
	
	int depth = 0;
   	int	forward = 0;

	int aim = 0;
	int aim_depth = 0;

	FILE *f = fopen(argv[1], "r");
	char c = fgetc(f); 
	std::string command;
	while (c != EOF) {
		if (c == '\n') {
			auto token_space = command.find(" ");
			if (token_space != std::string::npos) {
				std::string op = command.substr(0, token_space);
				int amt = std::stoi(command.substr(token_space+1));
				if (op == "forward") {
					forward += amt;
					aim_depth += aim * amt;
				}
				if (op == "down") {
					depth += amt;
					aim += amt;
				}
				if (op == "up") {
					depth -= amt;
					aim -= amt;
				}
			} 
			command = "";
		}
		else command += c;
		c = fgetc(f);
	}

	std::cout << "depth " << depth << ", "
		<< "forward " << forward << ", "
		<< "product " << depth * forward << std::endl;

	std::cout << "using aim: depth " << aim_depth << ", "
		<< "forward " << forward << ", "
		<< "aim " << aim << ", "
		<< "product " << aim_depth * forward << std::endl;

	return 0;
}
