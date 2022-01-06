#include <iostream>
#include <fstream>
#include <vector>

void split (std::vector<std::string> &vec, std::string str, std::string del) {
	auto token = str.find(del);
	if (token != std::string::npos) {
		vec.push_back(str.substr(0, token));
		split(vec, str.substr(token+del.size()), del);
	} else { vec.push_back(str); }
}

class Board {
	public:
	Board(std::string repr) { 
		std::vector<std::string> lines;
		split(lines, repr, "\n");
		for (auto line : lines) {
			std::vector<std::string> strcells;
			split(strcells, line, " ");
			size = 0;
			for (auto strcell : strcells) {
				if (strcell.size() == 0) continue;
				cells.push_back(std::stoi(strcell));
				state.push_back(0);
				size++;
			}
		}

	}

	int size;
	std::vector<int> cells;
	std::vector<int> state;

	void mark (int ex) {
		for (int y=0; y<size; y++) {
			for (int x=0; x<size; x++) {
				int i = x + y*size;
				if (cells[i] == ex) {
					state[i] = 1;
				}
			}
		}
	}

	bool win () {
		for (int y=0; y<size; y++) {
			int sum_row = 0;
			for (int x=0; x<size; x++) {
				int i = x + y*size;
				sum_row += state[i];
			}
			if (sum_row == size) return true;
		}
		for (int x=0; x<size; x++) {
			int sum_col = 0;
			for (int y=0; y<size; y++) {
				int i = x + y*size;
				sum_col += state[i];
			}
			if (sum_col == size) return true;
		}
		return false;
	}

	int score () {
		int sum = 0;
		for (int y=0; y<size; y++) {
			for (int x=0; x<size; x++) {
				int i = x + y*size;
				if (state[i] == 0) {
					sum += cells[i];
				}
			}
		}
		return sum;
	}
};

int main (int argc, char *argv[]) {
	if (argc != 2) return 1;
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');

	std::vector<int> extract;
	auto token = raw.find("\n");
	if (token != std::string::npos) {
		std::string ex = raw.substr(0, token);
		std::vector<std::string> vec;
	   	split(vec, ex, ",");
		for (auto v : vec) { extract.push_back(std::stoi(v)); }

		raw = raw.substr(token+2);
		if (raw[raw.size()-1] == '\n') raw = raw.substr(0, raw.size()-1); 
	} else return 1;

	std::vector<Board> boards;

	std::vector<std::string> str_boards;
	split(str_boards, raw, "\n\n");
	for (auto s : str_boards) {
		boards.emplace_back(s);
	}

	bool flag = false;
	std::vector<Board> filtered = boards;
	for (int ex : extract) {
		std::cout << ex << " " << filtered.size() << std::endl;
		std::vector<Board> next;
		int size = filtered.size(), seen = 0;;
		for (auto& board : filtered) {
			board.mark(ex);
			if (board.win() && filtered.size() != 1) {
				if (!flag) {
					std::cout << "first winner product: " << ex * board.score() << ", "
						<< "ex: " << ex << ", score: " << board.score() << std::endl;
					flag = true;
				}
			} else {
				next.push_back(board);
			}
		}
		if (next.size() == 1 && next[0].win()) {
			int score = next[0].score();
			std::cout << "last winner product: " << ex * score << ", "
				<< "ex: " << ex << ", score: " << score << std::endl;
			break;
		}
		filtered = next;
	}

	return 0;
}
