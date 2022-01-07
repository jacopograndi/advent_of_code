#include <iostream>
#include <fstream>
#include <vector>
#include <string>

class State {
	public: State() { iter = 0; booms = 0; }
	int size;
	std::vector<int> light;
	int xytoi (int x, int y) { return x+y*size; }
	int itox (int i) { return i % size; }
	int itoy (int i) { return i / size; }
	int iter;
	int booms;

	bool oob (int x, int y) {
		if (x < 0 || x >= size) return true;
		if (y < 0 || y >= size) return true;
		return false;
	}

	bool calm () {
		for (std::size_t i=0; i<light.size(); i++) {
			if (light[i] > 9) return false;
		}	
		return true;
	}

	void propagate (int x, int y) {
		for (int i=-1; i<2; i++) {
			for (int j=-1; j<2; j++) {
				if (!oob(x+i, y+j)) {
					light[xytoi(x+i, y+j)]++;
				}
			}
		}
	}

	void solve () {
		for (int y=0; y<size; y++) {
			for (int x=0; x<size; x++) {
				int i = xytoi(x, y);
				if (light[i] > 9) {
					propagate(x, y);
					booms ++;
					light[i] = -10000000;
					solve();
					return;
				}
			}
		}
		if (!calm()) solve();
	}

	void step () {
		for (int y=0; y<size; y++) {
			for (int x=0; x<size; x++) {
				int i = xytoi(x, y);
				light[i] ++;
			}
		}
		solve();
		for (std::size_t i=0; i<light.size(); i++) {
			if (light[i] < 0) light[i] = 0;
		}	
		iter ++;
	}

	void show () {
		std::cout << "iter: " << iter << std::endl;
		for (int y=0; y<size; y++) {
			for (int x=0; x<size; x++) {
				std::cout << light[xytoi(x, y)];
			}
			std::cout << std::endl;
		}
		std::cout << std::endl;
	}
};

int main (int argc, char * argv[]) {
	std::string line;
	std::vector<std::string> lines;
	std::ifstream f { argv[1] };
	do {
		std::getline(f, line);
		if (line.size() > 1) lines.push_back(line);
	} while (!f.eof());

	State state;
	for (auto l : lines) {
		state.size = 0;
		for (char c : l) {
			state.light.push_back(std::stoi(std::string { c }));
			state.size ++;
		}
	}

	state.show();
	int last = 0;
	int iters = argc > 2 ? atoi(argv[2]) : 100;
	for (int i=0; i<iters; i++) {
		state.step();
		state.show();

		if (state.booms - last == 100) {
			break;
		}
		last = state.booms;
	}

	std::cout << "flashes: " << state.booms << std::endl;

	return 0;
}
