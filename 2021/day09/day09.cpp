#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>

#include "../utils.h"

int xytoi (int size, int x, int y) { return size*y + x; }
int itox (int size, int i) { return i % size; }
int itoy (int size, int i) { return i / size; }
bool oob (int sizex, int sizey, int x, int y) {
	if (x < 0 || y < 0) return true;
	if (x >= sizex || y >= sizey) return true;
	return false;
}

std::vector<std::pair<int, int>> get_dirs() {
	std::vector<std::pair<int, int>> dirs {
		{ 1, 0 }, { 0, 1 }, { -1, 0 }, { 0, -1 }
	};
	return dirs;
}

bool is_min (std::vector<int> grid, int size, int i) {
	int x = itox(size, i);
	int y = itoy(size, i);
	for (auto dir : get_dirs()) {
		int j = xytoi(size, dir.first+x, dir.second+y);
		if (!oob(size, grid.size()/size, dir.first+x, dir.second+y))
			if (grid[j] <= grid[i])	
				return false;
	}
	return true;
}

std::vector<int> flood (std::vector<int> grid, int size, int i) {
	std::vector<int> visit ;
	std::vector<int> q { i };
	while (q.size() > 0) {
		int x = itox(size, q[0]);
		int y = itoy(size, q[0]);
		if (std::find(std::begin(visit), std::end(visit), q[0]) == std::end(visit)) 
			visit.push_back(q[0]);
		q.erase(std::begin(q));
		for (auto dir : get_dirs()) {
			int j = xytoi(size, dir.first+x, dir.second+y);
			if (std::find(std::begin(visit), std::end(visit), j) != std::end(visit)) 
				continue;
			if (!oob(size, grid.size()/size, dir.first+x, dir.second+y)) {
				if (grid[j] < 9) {
					q.push_back(j);
				}
			}
		}
	}
	return visit;
}

int main (int argc, char * argv[]) {
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	std::vector<std::string> lines;
	split(lines, raw, "\n");

	int size = 0;
	std::vector<int> grid;
	for (std::string line : lines) {
		if (line.size() == 0) continue;
		size = 0;
		for (char c : line) {
			grid.push_back(std::stoi(std::string { c } ));
			size++;
		}
	}

	std::cout << "lowpoints: ";
	std::vector<int> lows;
	int sum = 0;
	for (std::size_t i=0; i<grid.size(); i++) {
		if (is_min(grid, size, i)) {
			std::cout << "(" << itox(size, i) << " " << itoy(size, i) << "), ";
			sum += grid[i]+1;
			lows.push_back(i);
		}
	}

	std::cout << std::endl;
	std::cout << "sum of risk: " << sum << std::endl;

	std::vector<int> isles;
	for (auto low : lows) {
		auto visit = flood(grid, size, low);
		isles.push_back(visit.size());
	}
	std::sort(std::begin(isles), std::end(isles), std::greater<int>());

	int res = isles[0] * isles[1] * isles[2];
	std::cout << "three largest basins multiplied: " << res << std::endl;
	return 0;
}
