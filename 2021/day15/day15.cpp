#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <map>
#include <queue>
#include <functional>
#include "../utils.h"

class Grid {
	public: Grid () { }
	int xytoi (int x, int y) { return x+ y*sizex; }
	int itox (int i) { return i % sizex; }
	int itoy (int i) { return i / sizex; }
	bool oob (int x, int y) {
		if (x < 0 || x > sizex-1) return true;
		if (y < 0 || y > sizey-1) return true;
		return false;
	}

	int sizex, sizey;
	std::vector<int> cells;

	std::vector<int> fw (int i) {
		int x = itox(i), y = itoy(i);
		std::vector<int> star;
		for (int t=-1; t<2; t++) {
			for (int s=-1; s<2; s++) {
				if (abs(s)+abs(t) == 1 && !oob(x+s, y+t)) {
					star.push_back(xytoi(x+s, y+t));
				}
			}
		}
		return star;
	}

	int astar (int istart, int iend) {
		std::map<int, int> cost;
		std::map<int, int> prev;
		cost[iend] = cells[iend];
		prev[iend] = iend;
		auto cmp = [&cost] (int a, int b) { return cost[a] > cost[b]; };
		std::priority_queue<int, std::vector<int>, decltype(cmp)> Q(cmp);
		Q.push(iend);

		while (Q.size() > 0) {
			int n = Q.top(); Q.pop();
			auto star = fw(n);
			int min = cost[prev[n]];
			for (int j : star) {
				if (prev.find(j) == prev.end()) {
					prev[j] = n;
					cost[j] = cost[prev[j]] + cells[j];
					Q.push(j);
				}
				if (min >= cost[j]) {
					prev[n] = j;
					cost[n] = cost[prev[n]] + cells[n];
					min = cost[j];
				}
			}
		}

		if (true) {
			for (int y=0; y<sizey; y++) {
				for (int x=0; x<sizex; x++) {
					int n = xytoi(x, y);
					int i = itox(prev[n]);
					int j = itoy(prev[n]);
					std::cout << cells[n];
				}
				std::cout << std::endl;
			}
		}

		return cost[istart] - cells[istart];
	}
};

Grid five (Grid grid) {
	Grid big;
	big.sizex = grid.sizex * 5;
	big.sizey = grid.sizey * 5;
	for (int j=0; j<5; j++) {
		for (int y=0; y<grid.sizey; y++) {
			for (int i=0; i<5; i++) {
				for (int x=0; x<grid.sizex; x++) {
					int k = grid.cells[grid.xytoi(x, y)]+(i+j);
					while (k > 9) k -= 9;
					big.cells.push_back(k);
				}
			}
		}
	}
	return big;
}

int main (int argc, char *argv[]) {
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	std::vector<std::string> lines;
	split(lines, raw, "\n");

	Grid grid;
	grid.sizey = 0;
	for (std::string line : lines) {
		if (line.size() == 0) continue;
		grid.sizex = 0;
		for (char c : line) {
			grid.cells.push_back(std::stoi(std::string { c }));
			grid.sizex ++;
		}
		grid.sizey ++;
	}

	if (argc > 2) grid = five(grid);

	int l = grid.astar(grid.xytoi(0, 0), grid.xytoi(grid.sizex-1, grid.sizey-1));
	std::cout << "min risk: " << l << std::endl;

	return 0;
}
